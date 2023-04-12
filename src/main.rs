use std::fs;

use anyhow::anyhow;
use clap::Parser;
use image::{ColorType, GenericImageView, RgbaImage};
use serde::Serialize;

/// Bake krita animation render output to a sprite atlas + metadata
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
    /// on success, remove source render images
    #[clap(long)]
    rm: bool,
}

#[derive(Serialize)]
pub struct Flippy {
    /// the timings (when to start) each frame
    pub timings: Vec<u32>,
    /// the length of the entire animation
    pub length: u32,
}

mod titan_format {
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct SpriteSheetManifest {
        /// Path to the spritesheet image asset.
        pub path: String,
        /// Width and height of a tile inside the spritesheet.
        pub tile_size: Rect,
        /// How many columns of tiles there are inside the spritesheet.
        pub columns: usize,
        /// How many rows of tiles there are inside the spritesheet.
        pub rows: usize,
        #[serde(default)]
        /// Padding between tiles.
        pub padding: Option<Rect>,
        #[serde(default)]
        /// Offset from the top left from where the tiling begins.
        pub offset: Option<Rect>,
    }

    #[derive(Debug, Serialize)]
    pub struct Rect {
        pub w: f32,
        pub h: f32,
    }
}

fn main() -> anyhow::Result<()> {
    let Args {
        source_path,
        length: min_length,
        output,
        rm,
    } = Args::parse();
    println!("Generating sprite atlas from render folder {source_path:?}");

    let mut image_files = Vec::new();
    for file in fs::read_dir(&source_path)? {
        let file = file?;
        image_files.push(file);
    }

    let mut images = Vec::new();

    for file in image_files.iter() {
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

    let atlas_path = format!("{output_base_name}.png");
    image::save_buffer(
        &atlas_path,
        &atlas_image,
        atlas_image.width(),
        atlas_image.height(),
        ColorType::Rgba8,
    )?;
    println!("generated {atlas_path}");

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

    let flippy_path = format!("{output_base_name}.flippy");
    let flippy_ron = ron::to_string(&flippy)?;
    fs::write(&flippy_path, flippy_ron)?;
    println!("generated {flippy_path}");

    let titan = titan_format::SpriteSheetManifest {
        path: atlas_path,
        tile_size: titan_format::Rect {
            w: w as f32,
            h: h as f32,
        },
        columns: images.len(),
        rows: 1,
        padding: None,
        offset: None,
    };
    let titan_path = format!("{output_base_name}.titan");
    let titan_ron = ron::to_string(&titan)?;
    fs::write(&titan_path, titan_ron)?;
    println!("generated {titan_path}");

    if rm {
        println!("removing source images...");
        for file in image_files {
            fs::remove_file(file.path())?;
        }
    }

    Ok(())
}
