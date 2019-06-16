mod builder;
mod font;
mod load;
mod model;
mod result;
mod save;
mod scroll;
mod source;

use std::process::{Command, Stdio};

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use failure::format_err;

use crate::builder::ArrayBuilder;
use crate::font::FontDir;
use crate::model::dot_array::DotArray;
use crate::result::Result;
use crate::scroll::save_scroll;
use crate::source::Source;

fn main() {
    if let Err(err) = wrapped_main() {
            eprintln!("[Error] {}", err);
            std::process::exit(1);
    }
}

fn wrapped_main() -> Result<()> {
    let matches = App::new(crate_name!())
        .author(crate_authors!(", "))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("source")
                .takes_value(true)
                .max_values(1)
                .required(true),
        )
        .arg(
            Arg::with_name("ffmpeg")
                .long("ffmpeg")
                .takes_value(true),
        )
        .get_matches_safe()?;

    let source_file = matches.value_of("source").unwrap();
    let source = Source::load(source_file)?;

    let meta = source.metadata();

    let font_dir = FontDir::new("./fonts", meta.font())?;
    let color_set = meta.color_set()?;
    let background = color_set.get("_background")?.clone();

    let mut builder = ArrayBuilder::new(color_set, font_dir);
    for cmd in source.body() {
        builder.process(cmd)?;
    }
    let arr = builder.finish();

    let dot = load::load(meta.dot())?;
    let dot_arr = DotArray::new(arr, dot);

    let out_name = matches
        .value_of("output")
        .map(|s| s.to_string())
        .or_else(|| source.output())
        .unwrap_or_else(|| {
            source_file
                .trim_end_matches(".yaml")
                .trim_end_matches(".yml")
                .trim_end_matches(".json")
                .to_string()
        });

    if let Some(scroll) = meta.scroll() {
        save_scroll(&out_name, &dot_arr, scroll, &background)?;

        if let Some(fps_str) = matches.value_of("ffmpeg") {
            let mp4_name = format!("{}.mp4", out_name);
            let fps = fps_str.parse::<usize>()?;

            eprintln!("====vv ffmpeg encoding vv====");

            let output = Command::new("ffmpeg")
                .args(&[
                    "-y",
                    "-r",
                    &fps.to_string(),
                    "-i",
                    &format!("{}/%08d.png", out_name),
                    "-vcodec",
                    "libx264",
                    "-pix_fmt",
                    "yuv420p",
                    &mp4_name,
                ])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?;

            eprintln!("====^^ ffmpeg encoding ^^====");

            if !output.status.success() {
                return Err(format_err!(
                    "ffmpeg encoding failed({})",
                    output
                        .status
                        .code()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "?".to_string())
                ));
            }
        }
    } else {
        let dir_name = if !out_name.ends_with(".png") {
            out_name.to_string()
        } else {
            out_name + ".png"
        };

        save::save_image(dir_name, &dot_arr)?;
    }

    Ok(())
}
