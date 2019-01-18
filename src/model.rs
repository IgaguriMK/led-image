use std::hash::{Hash, Hasher};

use css_color_parser::Color as CssColor;
use failure::Error;


#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn parse(s: &str) -> Result<Option<Color>, Error> {
        if s.chars().nth(0) != Some('#') {
            return Ok(None);
        }

        let ccol = s.parse::<CssColor>()?;
        Ok(Some(Color{
            r: (ccol.r as f32) / 255.0,
            g: (ccol.b as f32) / 255.0,
            b: (ccol.b as f32) / 255.0,
            a: ccol.a,
        }))
    }

    pub fn blend(f: &Color, b: &Color) -> Color {
        let a = f.a + b.a * (1.0 - f.a);

        if a == 0.0 {
            return Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }.normalize();
        }

        Color{
            r: (f.r * f.a + b.r * b.a * (1.0 - f.a)) / a,
            g: (f.g * f.a + b.g * b.a * (1.0 - f.a)) / a,
            b: (f.b * f.a + b.b * b.a * (1.0 - f.a)) / a,
            a,
        }.normalize()
    }

    pub fn cross(base: &Color, p: &Color) -> Color {
        Color{
            r: base.r * p.a,
            g: base.g * p.g,
            b: base.b * p.b,
            a: base.a * p.a,
        }
    }

    fn normalize(self) -> Color {
        Color{
            r: self.r.max(0.0).min(1.0),
            g: self.g.max(0.0).min(1.0),
            b: self.b.max(0.0).min(1.0),
            a: self.a.max(0.0).min(1.0),
        }
    }

    pub fn r(&self) -> u8 {
        (255.0 * self.r) as u8
    }

    pub fn g(&self) -> u8 {
        (255.0 * self.g) as u8
    }

    pub fn b(&self) -> u8 {
        (255.0 * self.b) as u8
    }

    pub fn a(&self) -> u8 {
        (255.0 * self.a) as u8
    }
}

impl Eq for Color {}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r().hash(state);
        self.g().hash(state);
        self.b().hash(state);
        self.a().hash(state);
    }
}