#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use css_color_parser::Color as CssColor;
use failure::format_err;

use crate::result::Result;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new<Vr, Vg, Vb, Va>(r: Vr, g: Vg, b: Vb, a: Va) -> Color
    where
        Vr: ColorValue,
        Vg: ColorValue,
        Vb: ColorValue,
        Va: ColorValue,
    {
        Color {
            r: r.to_f32(),
            g: g.to_f32(),
            b: b.to_f32(),
            a: a.to_f32(),
        }
    }

    pub fn parse(s: &str) -> Result<Option<Color>> {
        if !s.starts_with('#') && !s.starts_with('@') {
            return Ok(None);
        }

        let s = s.trim_start_matches('@');

        let ccol = s.parse::<CssColor>()?;
        Ok(Some(Color::new(ccol.r, ccol.g, ccol.b, ccol.a)))
    }

    pub fn cross(&self, p: &Color) -> Color {
        Color {
            r: self.r * p.r,
            g: self.g * p.g,
            b: self.b * p.b,
            a: self.a * p.a,
        }
        .normalize()
    }

    fn normalize(self) -> Color {
        Color {
            r: self.r.max(0.0).min(1.0),
            g: self.g.max(0.0).min(1.0),
            b: self.b.max(0.0).min(1.0),
            a: self.a.max(0.0).min(1.0),
        }
    }

    pub fn r<V: ColorValue>(&self) -> V {
        V::from_f32(self.r)
    }

    pub fn g<V: ColorValue>(&self) -> V {
        V::from_f32(self.g)
    }

    pub fn b<V: ColorValue>(&self) -> V {
        V::from_f32(self.b)
    }

    pub fn a<V: ColorValue>(&self) -> V {
        V::from_f32(self.a)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r::<u8>() == other.r::<u8>()
            && self.g::<u8>() == other.g::<u8>()
            && self.b::<u8>() == other.b::<u8>()
            && self.a::<u8>() == other.a::<u8>()
    }
}

impl Eq for Color {}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r::<u8>().hash(state);
        self.g::<u8>().hash(state);
        self.b::<u8>().hash(state);
        self.a::<u8>().hash(state);
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#{:02x}{:02x}{:02x}{:02x}",
            self.r::<u8>(),
            self.g::<u8>(),
            self.b::<u8>(),
            self.a::<u8>()
        )
    }
}

#[derive(Debug, Clone)]
pub struct ColorSet {
    set: HashMap<String, Color>,
}

impl ColorSet {
    pub fn new(set: HashMap<String, Color>) -> ColorSet {
        ColorSet { set }
    }

    pub fn get(&self, name: &str) -> Result<&Color> {
        self.set
            .get(name)
            .ok_or_else(|| format_err!("unknown color: '{}'", name.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct ColorSetBuilder {
    set: HashMap<String, ColorEntry>,
}

impl ColorSetBuilder {
    pub fn new() -> ColorSetBuilder {
        ColorSetBuilder {
            set: HashMap::new(),
        }
    }

    pub fn append(&mut self, name: String, color: String) -> Result<()> {
        if let Some(c) = Color::parse(&color)? {
            self.set.insert(name, ColorEntry::Actual(c));
        } else {
            self.set.insert(name, ColorEntry::Name(color));
        }

        Ok(())
    }

    pub fn build(&mut self) -> Result<ColorSet> {
        loop {
            let unsolved: Vec<(String, String)> = self
                .set
                .iter()
                .filter_map(|(n, c)| match c {
                    ColorEntry::Actual(_) => None,
                    ColorEntry::Name(r) => Some((n.to_string(), r.to_string())),
                })
                .collect();

            if unsolved.is_empty() {
                return Ok(ColorSet::new(
                    self.set
                        .iter()
                        .map(|(n, c)| match c {
                            ColorEntry::Actual(c) => (n.to_string(), c.clone()),
                            _ => panic!("all entries should be ColorEntry::Actual."),
                        })
                        .collect(),
                ));
            }

            for (n, r) in unsolved {
                if n == r {
                    return Err(format_err!("color definision looped: {}", n));
                }

                if let Some(c) = self.set.get(&r) {
                    let c = c.clone();
                    self.set.insert(n, c);
                } else {
                    return Err(format_err!("unknown color: {}", n));
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum ColorEntry {
    Actual(Color),
    Name(String),
}

pub trait ColorValue {
    fn to_f32(self) -> f32;
    fn from_f32(v: f32) -> Self;
}

impl ColorValue for u8 {
    fn to_f32(self) -> f32 {
        f32::from(self) / f32::from(Self::max_value())
    }

    fn from_f32(v: f32) -> Self {
        (v * f32::from(Self::max_value())) as Self
    }
}

impl ColorValue for f32 {
    fn to_f32(self) -> f32 {
        self
    }

    fn from_f32(v: f32) -> Self {
        v
    }
}
