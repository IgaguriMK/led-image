pub mod array;
pub mod color;
pub mod command;

pub trait Imageable {
    fn dim(&self) -> (usize, usize);
    fn get(&self, x: usize, y: usize) -> &color::Color;
}
