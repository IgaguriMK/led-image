mod builder;
mod font;
mod load;
mod model;
mod result;
mod save;
mod source;

use crate::result::Result;
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

    let font_dir = font::FontDir::new("./fonts", meta.font())?;
    let color_set = meta.color_set()?;

    let mut builder = builder::ArrayBuilder::new(color_set, font_dir);
    for cmd in source.body() {
        builder.process(&cmd.into())?;
    }
    let arr = builder.finish();

    let dot = load::load(meta.dot())?;
    let dot_arr = model::dot_array::DotArray::new(arr, dot);

    save::save_image(&dot_arr, "./sample.png")?;

    Ok(())
}
