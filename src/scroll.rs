use std::fs;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::model::color::Color;
use crate::model::dot_array::DotArray;
use crate::result::Result;
use crate::save;
use crate::source::Scroll;

pub fn save_scroll(
    dir_path: impl AsRef<Path>,
    dot_arr: &DotArray,
    scroll: &Scroll,
    background: &Color,
) -> Result<()> {
    let _ = fs::remove_dir_all(&dir_path);
    fs::create_dir_all(&dir_path)?;

    let dot_width = dot_arr.dim_in_dot().0;
    let width = scroll.width() as isize;

    let (start, end) = (-width, (dot_width) as isize + width);
    let offsets: Vec<(PathBuf, DotArray)> = (start..end)
        .enumerate()
        .map(|(i, offset)| {
            let file_name = format!("{:08}.png", i);
            let path = dir_path.as_ref().join(file_name);
            let arr_slice = dot_arr.slice(offset, scroll.width(), background);
            (path, arr_slice)
        })
        .collect();

    offsets
        .into_par_iter()
        .try_for_each(|(path, arr_slice)| save::save_image(path, &arr_slice))?;

    Ok(())
}
