use std::path::Path;

use image::{ImageBuffer, Rgba};

use crate::model::Imageable;
use crate::result::Result;

pub fn save_image(img: impl Imageable, path: impl AsRef<Path>) -> Result<()> {
    let (w, h) = img.dim();

    let img = ImageBuffer::from_fn(w as u32, h as u32, |x, y| {
        let c = img.get(x as usize, y as usize);
        Rgba([c.r(), c.g(), c.b(), c.a()])
    });

    img.save(path)?;

    Ok(())
}