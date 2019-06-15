pub mod array;
pub mod color;
pub mod command;
pub mod dot_array;

use color::Color;

pub trait Imageable {
    fn dim(&self) -> (usize, usize);
    fn get(&self, x: usize, y: usize) -> Color;
}
