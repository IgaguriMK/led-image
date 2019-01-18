use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use failure::{format_err, Error};
use image::GenericImageView;

#[derive(Debug, Clone)]
pub struct FontDir {
    dir: PathBuf,
    cache: HashMap<char, CharImage>,
}

impl FontDir {
    pub fn new(search_path: impl AsRef<Path>, font_name: &str) -> FontDir {
        let mut dir = PathBuf::new();
        dir.push(search_path);
        dir.push(font_name);
        dir.push("chars");

        FontDir {
            dir,
            cache: HashMap::new(),
        }
    }

    pub fn get_char(&mut self, ch: char) -> Result<&CharImage, Error> {
        if !self.cache.contains_key(&ch) {
            let mut file_name = self.dir.clone();
            file_name.push(format!("{:04x}.png", ch as u16));

            self.cache.insert(ch, CharImage::load(file_name)?);
        }

        Ok(self.cache.get(&ch).unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct CharImage {
    ysize: usize,
    arr: Vec<u64>,
}

impl CharImage {
    fn new(xsize: usize, ysize: usize) -> CharImage {
        CharImage {
            ysize,
            arr: vec![0; xsize],
        }
    }

    fn set(&mut self, x: usize, y: usize, v: bool) {
        if y >= self.ysize {
            panic!("y: out of index");
        }

        let &current = self.arr.get(x).expect("x: out of index");
        let mask = 1 << y;

        if v {
            self.arr[x] = current | mask;
        } else {
            self.arr[x] = current & !mask;
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if y >= self.ysize {
            panic!("y: out of index");
        }

        let &current = self.arr.get(x).expect("x: out of index");
        let mask = 1 << y;

        current & mask != 0
    }

    fn load(path: impl AsRef<Path>) -> Result<CharImage, Error> {
        let img = image::open(path)?.to_luma();

        let (xsize, ysize) = img.dimensions();

        let mut cimg = CharImage::new(xsize as usize, ysize as usize);

        for (x, y, pixcel) in img.enumerate_pixels() {
            if pixcel.data[0] < 8 {
                cimg.set(x as usize, y as usize, true);
            }
        }

        Ok(cimg)
    }
}
