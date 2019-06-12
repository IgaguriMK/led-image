use failure::format_err;

use crate::font::FontDir;
use crate::model::array::Array;
use crate::model::color::ColorSet;
use crate::model::command::{Command, Space, Text};
use crate::result::Result;

#[derive(Debug)]
pub struct ArrayBuilder {
    array: Array,
    color_set: ColorSet,
    font_dir: FontDir,
}

impl ArrayBuilder {
    pub fn new(color_set: ColorSet, font_dir: FontDir) -> ArrayBuilder {
        let height = font_dir.height();

        ArrayBuilder {
            array: Array::new(height),
            color_set,
            font_dir,
        }
    }

    pub fn process(&mut self, command: &Command) -> Result<()> {
        match command {
            Command::Text(text) => self.process_text(text),
            Command::Space(space) => self.process_space(space),
        }
    }

    fn process_text(&mut self, text: &Text) -> Result<()> {
        let fore = self
            .color_set
            .get(text.foreground().unwrap_or("_foreground"))
            .map_err(|e| format_err!("unknown foreground color '{}'", e))?;
        let back = self
            .color_set
            .get(text.background().unwrap_or("_background"))
            .map_err(|e| format_err!("unknown background color '{}'", e))?;

        for c in text.content().chars() {
            let ch = self.font_dir.get_char(c)?;

            let (w, h) = ch.dim();
            if self.array.height() != h {
                return Err(format_err!("height mismatch"));
            }

            for x in 0..w {
                let mut line = Vec::new();

                for y in 0..h {
                    if ch.get(x, y) {
                        line.push(fore.clone());
                    } else {
                        line.push(back.clone());
                    }
                }

                self.array.add_line(line);
            }
        }

        Ok(())
    }

    fn process_space(&mut self, space: &Space) -> Result<()> {
        let back = self
            .color_set
            .get(space.background().unwrap_or("_background"))
            .map_err(|e| format_err!("unknown background color '{}'", e))?;

        let mut line = Vec::new();
        line.resize_with(self.array.height(), || back.clone());

        for _ in 0..space.width() {
            self.array.add_line(line.clone());
        }

        Ok(())
    }

    pub fn finish(self) -> Array {
        self.array
    }
}
