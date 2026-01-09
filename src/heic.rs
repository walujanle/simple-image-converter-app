//! HEIC/HEIF image format decoder using libheif.

use anyhow::Result;
use image::{DynamicImage, ImageBuffer, Rgba};
use libheif_rs::{ColorSpace, DecodingOptions, HeifContext, LibHeif, RgbChroma};
use std::path::Path;

/// Decodes HEIC/HEIF image file to DynamicImage.
pub fn load_heic_via_libheif(path: &Path) -> Result<(DynamicImage, Option<Vec<u8>>)> {
    let path_str = path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid path encoding"))?;

    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file(path_str)
        .map_err(|e| anyhow::anyhow!("Failed to read HEIC file: {}", e))?;
    let handle = ctx
        .primary_image_handle()
        .map_err(|e| anyhow::anyhow!("No primary image: {}", e))?;

    let image = lib_heif
        .decode(
            &handle,
            ColorSpace::Rgb(RgbChroma::Rgba),
            None::<DecodingOptions>,
        )
        .map_err(|e| anyhow::anyhow!("Decoding failed: {}", e))?;

    let width = image.width();
    let height = image.height();
    let planes = image.planes();
    let interleaved = planes
        .interleaved
        .ok_or_else(|| anyhow::anyhow!("No interleaved plane found"))?;

    let data = interleaved.data;
    let stride = interleaved.stride;
    let mut buffer = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..height {
        let start = (y as usize) * stride;
        let end = start + (width as usize) * 4;
        buffer.extend_from_slice(&data[start..end]);
    }

    let img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, buffer)
        .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer"))?;

    Ok((DynamicImage::ImageRgba8(img_buffer), None))
}
