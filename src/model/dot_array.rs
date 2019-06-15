use std::collections::{HashMap, HashSet};

use super::array::Array;
use super::color::Color;
use super::Imageable;

#[derive(Debug)]
pub struct DotArray {
    array: Array,
    dot_table: HashMap<Color, Array>,
    dot_dim: (usize, usize),
}

impl DotArray {
    pub fn new(array: Array, dot: Array) -> DotArray {
        let mut colors = HashSet::new();

        for (_, _, c) in array.iter() {
            colors.insert(c.clone());
        }

        let mut dot_table = HashMap::new();

        for c in colors.into_iter() {
            let c_dot = dot.map(|p| p.cross(&c));
            dot_table.insert(c, c_dot);
        }

        DotArray {
            array,
            dot_table,
            dot_dim: dot.dim(),
        }
    }

    pub fn dim_in_dot(&self) -> (usize, usize) {
        self.array.dim()
    }

    pub fn slice<'a>(&'a self, offset: isize, width: usize, background: &'a Color) -> DotArray {
        let height = self.array.height();

        let blank_line: Vec<Color> = (0..height).map(|_| background.clone()).collect();

        let mut new_array = Array::with_dim(width, height);

        for i in 0..width {
            let real_x = (i as isize) + offset;

            if real_x < 0 {
                new_array.add_line(blank_line.clone());
            } else {
                new_array.add_line(
                    self.array
                        .get_line(real_x as usize)
                        .cloned()
                        .unwrap_or_else(|| blank_line.clone()),
                );
            }
        }

        DotArray {
            array: new_array,
            dot_table: self.dot_table.clone(),
            dot_dim: self.dot_dim,
        }
    }
}

impl Imageable for DotArray {
    fn dim(&self) -> (usize, usize) {
        let (ax, ay) = self.array.dim();
        let (dx, dy) = self.dot_dim;
        (ax * dx, ay * dy)
    }

    fn get(&self, x: usize, y: usize) -> Color {
        let (dx, dy) = self.dot_dim;

        let ap = self.array.get(x / dx, y / dy);

        self.dot_table.get(&ap).unwrap().get(x % dx, y % dy).clone()
    }
}
