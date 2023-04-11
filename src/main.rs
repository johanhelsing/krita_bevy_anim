use anyhow::anyhow;
use clap::Parser;
use image::{ColorType, GenericImageView, RgbaImage};
use serde::{Deserialize, Serialize};

/// A program
#[derive(Parser)]
struct Args {
    /// Path to the directory containing the source images
    source_path: String,
    /// Base name of output
    #[clap(long)]
    output: Option<String>,
    /// length in frames
    #[clap(long, short)]
    length: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct Flippy {
    /// the timings (when to start) each frame
    pub timings: Vec<u32>,
    /// the length of the entire animation
    pub length: u32,
}

fn main() -> anyhow::Result<()> {
    let Args {
        source_path,
        length: min_length,
        output,
    } = Args::parse();

    let mut images = Vec::new();

    for file in std::fs::read_dir(&source_path)? {
        let file = file?;
        let image = image::open(file.path())?;
        let file_name = file.file_name();
        let frame = file_name
            .to_str()
            .unwrap()
            .strip_suffix(".png")
            .ok_or(anyhow!("not a png file"))?;

        let frame = frame.parse::<u32>()?;
        images.push((frame, image));
    }

    let first_image = &images.first().ok_or(anyhow!("no images"))?.1;

    let (w, h) = first_image.dimensions();

    // todo: figure out ideal dimensions to use, for now just a single row

    let mut atlas_image: RgbaImage = image::ImageBuffer::new(w * images.len() as u32, h);

    for (i, image) in images.iter().enumerate() {
        let x_offset = i as u32 * w;
        for y in 0..h {
            for x in 0..w {
                let p = image.1.get_pixel(x, y);
                atlas_image.put_pixel(x + x_offset, y, p);
            }
        }
    }

    // place next to the source dir with same name by default
    let output_base_name = output.unwrap_or(source_path);

    image::save_buffer(
        format!("{output_base_name}.png"),
        &atlas_image,
        atlas_image.width(),
        atlas_image.height(),
        ColorType::Rgba8,
    )?;

    let timings: Vec<_> = images.iter().map(|(f, _)| *f).collect();

    // if the last frame isn't unique, we don't know exactly how long the animation is.
    // allow a min_length variable so we can override it through command line args
    let last_unique_frame = timings.last().cloned().ok_or(anyhow!("no timings"))?;
    let length = if let Some(min_length) = min_length {
        min_length.max(last_unique_frame + 1)
    } else {
        last_unique_frame + 1
    };

    let flippy = Flippy { length, timings };

    let flippy_json = serde_json::to_string(&flippy)?;
    let flippy_path = format!("{output_base_name}.flippy");
    std::fs::write(flippy_path, flippy_json)?;

    Ok(())
}
