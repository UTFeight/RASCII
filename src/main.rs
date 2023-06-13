mod craiyon;
use std::io;

use ansi_term::Color::*;
use clap::Parser;
use image::DynamicImage;
use rascii_art::{
    charsets,
    RenderOptions,
};
use spinners::{
    Spinner,
    Spinners,
};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(help = "Path to the image")]
    filename: String,
    #[arg(
        short,
        long,
        help = "Width of the output image. Defaults to 128 if width and height are not specified"
    )]
    width: Option<u32>,
    #[arg(
        short = 'H',
        long,
        help = "Height of the output image, if not specified, it will be calculated to keep the \
                aspect ratio"
    )]
    height: Option<u32>,
    #[arg(
        name = "color",
        short,
        long,
        help = "Whether to use colors in the output image"
    )]
    colored: bool,
    #[arg(
        short,
        long,
        help = "Inverts the weights of the characters. Useful for white backgrounds"
    )]
    invert: bool,
    #[arg(short, long, help = "Use AI to generate ascii art")]
    query: Option<String>,
    #[arg(
        short = 'C',
        long,
        default_value = "default",
        help = "Characters used to render the image, from transparent to opaque. Built-in \
                charsets: block, emoji, default, russian, slight"
    )]
    charset: String,
}

fn save_images(images: Vec<DynamicImage>, name: &str) -> image::ImageResult<()> {
    for (i, image) in images.iter().enumerate() {
        let filename = format!("{}-{}.png", name, i);
        image.save(filename)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> image::ImageResult<()> {
    let mut args = Args::parse();

    let clusters = UnicodeSegmentation::graphemes(args.charset.as_str(), true).collect::<Vec<_>>();
    let charset = charsets::from_str(args.charset.as_str()).unwrap_or(clusters.as_slice());

    if args.width.is_none() && args.height.is_none() {
        args.width = Some(80);
    }

    if let Some(query) = args.query {
        println!("Generating...");
        let mut sp = Spinner::new(Spinners::Arc, query.clone());
        // let images = craiyon::generate(&query);
        // save_images(images.await.expect("Failed to construct images"), &query).expect("Couldn't save images");
        sp.stop_with_symbol("\x1b[32m✔\x1b[0m");
    }

    rascii_art::render_to(
        args.filename,
        &mut io::stdout(),
        RenderOptions {
            width: args.width,
            height: args.height,
            colored: args.colored,
            invert: args.invert,
            charset,
        },
    )?;

    Ok(())
}
