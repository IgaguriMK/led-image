use std::path::Path;

use failure::Error;

pub trait Font {
    type Gryph;

    fn new(search_path: impl AsRef<Path>, font_name: &str) -> Self;
    fn get_char(&mut self, ch: char) -> Result<&Self::Gryph, Error>;
}

pub trait Gryph {
    fn empty(xsize: usize, ysize: usize) -> Self;

    fn set(&mut self, x: usize, y: usize, v: bool);

    fn get(&self, x: usize, y: usize) -> bool;
}
