mod entities;

mod model;
mod source;
mod font;

use failure::Error;

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

fn wrapped_main() -> Result<(), Error> {
    let source = Source::load("./sample.yaml")?;
    println!("{:#?}", source);

    let mut font_dir = font::FontDir::new("./fonts", source.metadata().font());
    
    Ok(())
}