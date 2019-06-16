use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use serde_derive::Deserialize;
use serde_yaml::from_reader;

use crate::model::color::{ColorSet, ColorSetBuilder};
use crate::model::command;
use crate::result::Result;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    meta: Metadata,
    output: Option<String>,
    body: Vec<Command>,
}

impl Source {
    pub fn load(path: impl AsRef<Path>) -> Result<Source> {
        let f = File::open(path)?;
        Ok(from_reader(f)?)
    }

    pub fn metadata(&self) -> Metadata {
        self.meta.clone()
    }

    pub fn output(&self) -> Option<String> {
        if let Some(ref o) = self.output {
            Some(o.clone())
        } else {
            None
        }
    }

    pub fn body(&self) -> impl Iterator<Item = &Command> {
        self.body.iter()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    colors: HashMap<String, String>,
    dot: String,
    font: String,
    scroll: Option<Scroll>,
}

impl Metadata {
    pub fn font(&self) -> &str {
        self.font.as_str()
    }

    pub fn dot(&self) -> &str {
        self.dot.as_str()
    }

    pub fn color_set(&self) -> Result<ColorSet> {
        let mut builder = ColorSetBuilder::new();

        for (n, c) in self.colors.iter() {
            builder.append(n.to_string(), c.to_string())?;
        }

        builder.build()
    }

    pub fn scroll(&self) -> Option<&Scroll> {
        self.scroll.as_ref()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scroll {
    width: usize,
}

impl Scroll {
    pub fn width(&self) -> usize {
        self.width
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum Command {
    #[serde(rename = "text")]
    Text(Text),
    #[serde(rename = "space")]
    Space(Space),
}

impl<'a> command::Command<'a, Text, Space> for Command {
    fn when_text<U>(&'a self, none: U, f: impl FnOnce(&Text) -> U) -> U {
        if let Command::Text(text) = self {
            f(text)
        } else {
            none
        }
    }

    fn when_space<U>(&'a self, none: U, f: impl FnOnce(&Space) -> U) -> U {
        if let Command::Space(space) = self {
            f(space)
        } else {
            none
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Text {
    #[serde(rename = "c")]
    content: String,
    #[serde(rename = "f")]
    foreground: Option<String>,
    #[serde(rename = "b")]
    background: Option<String>,
}

impl command::Text for Text {
    fn content(&self) -> &str {
        &self.content
    }

    fn foreground(&self) -> Option<&str> {
        self.foreground.as_ref().map(|s| &**s)
    }

    fn background(&self) -> Option<&str> {
        self.background.as_ref().map(|s| &**s)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Space {
    #[serde(rename = "w")]
    width: usize,
    #[serde(rename = "b")]
    background: Option<String>,
}

impl command::Space for Space {
    fn width(&self) -> usize {
        self.width
    }

    fn background(&self) -> Option<&str> {
        self.background.as_ref().map(|s| &**s)
    }
}
