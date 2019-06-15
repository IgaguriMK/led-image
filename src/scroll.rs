use std::fs;
use std::path::Path;

use crate::model::color::Color;
use crate::model::dot_array::DotArray;
use crate::result::Result;
use crate::save;
use crate::source::Scroll;

pub fn save_scroll(
    dir_path: impl AsRef<Path>,
    base_name: &str,
    dot_arr: &DotArray,
    scroll: &Scroll,
    background: &Color,
) -> Result<()> {
    let _ = fs::remove_dir_all(&dir_path);
    fs::create_dir_all(&dir_path)?;

    let dot_width = dot_arr.dim_in_dot().0;
    let width = scroll.width() as isize;

    let (start, end) = (-width, (dot_width) as isize + width);

    let count = (end - start) as usize;
    for (i, offset) in (start..end).enumerate() {
        if i % 30 == 29 {
            eprintln!("{} ({:.1} %)", i, 100.0 * (i as f64) / (count as f64));
        }

        let file_name = format!("{}_{}.png", base_name, file_index(count, i));
        let path = dir_path.as_ref().join(file_name);

        let arr_slice = dot_arr.slice(offset, scroll.width(), background);
        save::save_image(path, &arr_slice)?;
    }

    Ok(())
}

fn file_index(count: usize, index: usize) -> String {
    if count < 10 {
        format!("{:01}", index)
    } else if count < 100 {
        format!("{:02}", index)
    } else if count < 1000 {
        format!("{:03}", index)
    } else if count < 10_000 {
        format!("{:04}", index)
    } else if count < 100_000 {
        format!("{:05}", index)
    } else {
        format!("{:09}", index)
    }
}