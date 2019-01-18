use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;

use failure::{Error, format_err};
use serde_derive::Deserialize;
use serde_yaml::from_reader;

use crate::model::Color;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    meta: Metadata,
    body: Vec<Command>,
}

impl Source {
    pub fn load(path: impl AsRef<Path>) -> Result<Source, Error> {
        let f = File::open(path)?;
        Ok(from_reader(f)?)
    }

    pub fn metadata(&self) -> &Metadata {
        &self.meta
    }

    pub fn body(&self) -> impl Iterator<Item=&Command> {
        self.body.iter()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    colors: HashMap<String, String>,
    font: String,
    #[serde(default)]
    scroll: bool,
}

impl Metadata {
    pub fn font(&self) -> &str {
        self.font.as_str()
    }

    pub fn scroll(&self) -> bool {
        self.scroll
    }

    pub fn get_color(&self, name: &str) -> Result<Color, Error> {
        let mut checked: HashSet<String> = HashSet::new();
        let mut color_name = name;
        
        loop {
            if let Some(c) = self.colors.get(color_name) {
                match Color::parse(c)? {
                    Some(color) => {
                        return Ok(color);
                    }
                    None => {
                        checked.insert(c.to_string());
                        color_name = c;
                    }
                }
            } else {
                return Err(format_err!("color not found"));
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum Command {
    #[serde(rename = "text")]
    Text(Text),
    #[serde(rename = "space")]
    Space(Space),
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

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Space {
    #[serde(rename = "w")]
    width: usize,
    #[serde(rename = "b")]
    background: Option<String>,
}

