//! # Usage:
//! ```no_run
//! use rascii_art::{
//!     render_to,
//!     RenderOptions,
//! };
//!
//! fn main() {
//!     let mut buf = String::new();
//!
//!     render_to(
//!         r"/path/to/image.png",
//!         &mut buf,
//!         &RenderOptions::new()
//!             .width(100)
//!             .colored(true)
//!             .charset(&[".", ",", "-", "*", "£", "$", "#"]),
//!     )
//!     .unwrap();
//! }
//! ```

pub mod charsets;

mod gif_renderer;
mod image_renderer;
mod renderer;

use std::{
    io,
    path::Path,
};
use expanduser::expanduser;
use image::DynamicImage;
use image_renderer::ImageRenderer;
pub use renderer::RenderOptions;
use renderer::Renderer;

pub fn render<P: AsRef<Path> + AsRef<str>>(
    path: P,
    to: &mut impl io::Write,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let absolute_path = expanduser(&path)?;
    let image = &image::open(absolute_path)?;
    render_image(image, to, &options)
}

pub fn render_image(
    image: &DynamicImage,
    to: &mut impl io::Write,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let renderer = ImageRenderer::new(image, options);
    renderer.render(to)?;
    Ok(())
}

pub fn render_to<P: AsRef<Path> + AsRef<str>>(
    path: P,
    buffer: &mut String,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let absolute_path = expanduser(&path)?;
    let image = &image::open(absolute_path)?;
    let renderer = ImageRenderer::new(image, options);
    renderer.render_to(buffer)?;
    Ok(())
}

pub fn render_image_to(
    image: &DynamicImage,
    buffer: &mut String,
    options: &RenderOptions<'_>,
) -> image::ImageResult<()> {
    let renderer = ImageRenderer::new(image, options);
    renderer.render_to(buffer)?;
    Ok(())
}
