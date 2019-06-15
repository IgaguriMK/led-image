use std::cell::RefCell;
use std::collections::HashMap;

use super::array::Array;
use super::color::Color;
use super::Imageable;

#[derive(Debug)]
pub struct DotArray {
    array: Array,
    dot: Array,
    dot_table: RefCell<HashMap<Color, Array>>,
}

impl DotArray {
    pub fn new(array: Array, dot: Array) -> DotArray {
        DotArray {
            array,
            dot,
            dot_table: RefCell::new(HashMap::new()),
        }
    }

    fn ensure_color_dot(&self, c: &Color) {
        if self.dot_table.borrow().contains_key(c) {
            return;
        }

        let new_dot = self.dot.map(|p| p.cross(c));
        self.dot_table.borrow_mut().insert(c.clone(), new_dot);
    }
}

impl Imageable for DotArray {
    fn dim(&self) -> (usize, usize) {
        let (ax, ay) = self.array.dim();
        let (dx, dy) = self.dot.dim();
        (ax * dx, ay * dy)
    }

    fn get(&self, x: usize, y: usize) -> Color {
        let (dx, dy) = self.dot.dim();

        let ap = self.array.get(x / dx, y / dy);

        self.ensure_color_dot(&ap);
        self.dot_table
            .borrow()
            .get(&ap)
            .map(|arr| arr.get(x % dx, y % dy).clone())
            .unwrap()
    }
}
