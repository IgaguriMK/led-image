mod builder;
mod font;
mod load;
mod model;
mod result;
mod save;
mod scroll;
mod source;

use crate::builder::ArrayBuilder;
use crate::font::FontDir;
use crate::model::dot_array::DotArray;
use crate::result::Result;
use crate::scroll::save_scroll;
use crate::source::Source;

fn main() {
    match wrapped_main() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("[Error] {}", err);
            std::process::exit(1);
        }
    }
}

fn wrapped_main() -> Result<()> {
    let source = Source::load("./sample.yaml")?;

    let meta = source.metadata();

    let font_dir = FontDir::new("./fonts", meta.font())?;
    let color_set = meta.color_set()?;
    let background = color_set.get("_background")?.clone();

    let mut builder = ArrayBuilder::new(color_set, font_dir);
    for cmd in source.body() {
        builder.process(&cmd.into())?;
    }
    let arr = builder.finish();

    let dot = load::load(meta.dot())?;
    let dot_arr = DotArray::new(arr, dot);

    if let Some(scroll) = meta.scroll() {
        save_scroll("sample", "sample", &dot_arr, scroll, &background)?;
    } else {
        save::save_image("./sample.png", &dot_arr)?;
    }

    Ok(())
}
