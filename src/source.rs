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

    pub fn body(self) -> impl Iterator<Item = Command> {
        self.body.into_iter()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    colors: HashMap<String, String>,
    dot: String,
    font: String,
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
}

#[derive(Debug, Clone, Deserialize)]
pub enum Command {
    #[serde(rename = "text")]
    Text(Text),
    #[serde(rename = "space")]
    Space(Space),
}

impl Into<command::Command> for Command {
    fn into(self) -> command::Command {
        match self {
            Command::Text(text) => command::Command::Text(text.into()),
            Command::Space(space) => command::Command::Space(space.into()),
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

impl Into<command::Text> for Text {
    fn into(self) -> command::Text {
        command::Text::new(self.content, self.foreground, self.background)
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

impl Into<command::Space> for Space {
    fn into(self) -> command::Space {
        command::Space::new(self.width, self.background)
    }
}
