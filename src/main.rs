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

    let font_dir = font::FontDir::new("./fonts", source.metadata().font())?;
    let color_set = source.metadata().color_set()?;

    let mut builder = builder::ArrayBuilder::new(color_set, font_dir);
    for cmd in source.body() {
        builder.process(&cmd.into())?;
    }
    let arr = builder.finish();

    let dot = load::load("./dot.png")?;

    save::save_image(arr, "./sample.png")?;

    Ok(())
}
