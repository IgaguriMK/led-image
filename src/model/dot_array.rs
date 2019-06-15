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

    pub fn dot_dim(&self) -> (usize, usize) {
        self.dot.dim()
    }

    pub fn dim_in_dot(&self) -> (usize, usize) {
        self.array.dim()
    }

    pub fn slice<'a>(
        &'a self,
        offset: isize,
        width: usize,
        background: &'a Color,
    ) -> DotArraySlice<'a> {
        DotArraySlice {
            dot_array: self,
            offset,
            width,
            background,
        }
    }

    fn ensure_color_dot(&self, c: &Color) {
        if self.dot_table.borrow().contains_key(c) {
            return;
        }

        let new_dot = self.dot.map(|p| p.cross(c));
        self.dot_table.borrow_mut().insert(c.clone(), new_dot);
    }

    fn get_with_dot_offset(&self, x: usize, y: usize, offset: isize, background: &Color) -> Color {
        let (dx, dy) = self.dot.dim();
        let real_dot_x = (x / dx) as isize + offset;

        if real_dot_x < 0 || self.array.dim().0 as isize <= real_dot_x {
            self.ensure_color_dot(background);
            return self
                .dot_table
                .borrow()
                .get(background)
                .map(|arr| arr.get(x % dx, y % dy).clone())
                .unwrap();
        }

        let ap = self.array.get(real_dot_x as usize, y / dy);

        self.ensure_color_dot(&ap);
        self.dot_table
            .borrow()
            .get(&ap)
            .map(|arr| arr.get(x % dx, y % dy).clone())
            .unwrap()
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

#[derive(Debug)]
pub struct DotArraySlice<'a> {
    dot_array: &'a DotArray,
    offset: isize,
    width: usize,
    background: &'a Color,
}

impl<'a> Imageable for DotArraySlice<'a> {
    fn dim(&self) -> (usize, usize) {
        let dot_xsize = self.dot_array.dot_dim().0;
        let da_ysize = self.dot_array.dim().1;
        (dot_xsize * self.width, da_ysize)
    }

    fn get(&self, x: usize, y: usize) -> Color {
        self.dot_array
            .get_with_dot_offset(x, y, self.offset, self.background)
    }
}
