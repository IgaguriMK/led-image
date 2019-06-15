use std::path::Path;

use crate::model::array::Array;
use crate::model::color::Color;
use crate::result::Result;

pub fn load(path: impl AsRef<Path>) -> Result<Array> {
    let img = image::open(path)?.to_rgba();
    let (xsize, ysize) = img.dimensions();

    let mut array = Array::with_dim(xsize as usize, ysize as usize);

    for x in 0..xsize {
        let mut line = Vec::with_capacity(ysize as usize);
        for y in 0..ysize {
            let p = img.get_pixel(x, y);
            line.push(Color::new(p.data[0], p.data[1], p.data[2], p.data[3]));
        }
        array.add_line(line);
    }

    Ok(array)
}
