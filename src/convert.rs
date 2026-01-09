//! Image conversion engine with format support for JPEG, PNG, WebP, and HEIC.

use crate::state::{ConversionOptions, ImageFormat};
use anyhow::{Context, Result};
use image::imageops::FilterType;
use image::DynamicImage;
use img_parts::jpeg::JpegSegment;
use img_parts::{ImageEXIF, ImageICC};
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

static SRGB_ICC: &[u8] = include_bytes!("srgb.icc");

struct Metadata {
    exif: Option<Vec<u8>>,
}

const MAGIC_JPEG: &[u8] = &[0xFF, 0xD8, 0xFF];
const MAGIC_PNG: &[u8] = &[0x89, 0x50, 0x4E, 0x47];
const MAGIC_WEBP: &[u8] = b"RIFF";
const MAGIC_HEIC: &[u8] = b"ftyp";

/// Validates file format by checking magic bytes at file header.
fn validate_file_magic(path: &PathBuf) -> Result<()> {
    let mut file = std::fs::File::open(path)?;
    let mut header = [0u8; 12];
    std::io::Read::read(&mut file, &mut header)?;

    let ext = path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();

    let valid = match ext.as_str() {
        "jpg" | "jpeg" => header.starts_with(MAGIC_JPEG),
        "png" => header.starts_with(MAGIC_PNG),
        "webp" => header.starts_with(MAGIC_WEBP) && &header[8..12] == b"WEBP",
        "heic" | "heif" => header[4..8] == *MAGIC_HEIC,
        _ => true,
    };

    if valid {
        Ok(())
    } else {
        anyhow::bail!("Invalid file format: magic bytes mismatch")
    }
}

/// Generates target filename based on conversion options and input path.
pub fn get_target_filename(input_path: &PathBuf, options: &ConversionOptions) -> String {
    let mut file_stem = input_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    if !options.find_pattern.is_empty() {
        file_stem = file_stem.replace(&options.find_pattern, &options.replace_with);
    }

    if options.auto_suffix {
        if let Ok((w, h)) = image::image_dimensions(input_path) {
            file_stem.push_str(&get_smart_suffix(w, h, options.quality, options.format));
        }
    }

    let ext = match options.format {
        ImageFormat::Jpeg => "jpg",
        ImageFormat::Png => "png",
        ImageFormat::WebP => "webp",
    };

    format!("{}{}.{}", options.prefix, file_stem, ext)
}

/// Extracts EXIF metadata from JPEG, WebP, or PNG files.
fn extract_metadata(input_path: &PathBuf) -> Option<Metadata> {
    let file_bytes = std::fs::read(input_path).ok()?;
    let bytes_owned = bytes::Bytes::from(file_bytes);
    let mut exif = None;

    if let Ok(jpeg) = img_parts::jpeg::Jpeg::from_bytes(bytes_owned.clone()) {
        for segment in jpeg.segments() {
            if segment.contents().starts_with(b"Exif\0\0") {
                exif = Some(segment.contents().to_vec());
            }
        }
    } else if let Ok(webp) = img_parts::webp::WebP::from_bytes(bytes_owned.clone()) {
        if let Some(e) = webp.exif() {
            let v = if e.starts_with(b"Exif\0\0") {
                e.to_vec()
            } else {
                let mut buf = Vec::new();
                buf.extend_from_slice(b"Exif\0\0");
                buf.extend_from_slice(&e);
                buf
            };
            exif = Some(v);
        }
    } else if let Ok(png) = img_parts::png::Png::from_bytes(bytes_owned) {
        for chunk in png.chunks() {
            if chunk.kind() == *b"eXIf" {
                let contents = chunk.contents();
                exif = Some(if contents.starts_with(b"Exif\0\0") {
                    contents.to_vec()
                } else {
                    let mut buf = Vec::new();
                    buf.extend_from_slice(b"Exif\0\0");
                    buf.extend_from_slice(&contents);
                    buf
                });
            }
        }
    }

    exif.map(|e| Metadata { exif: Some(e) })
}

/// Resets EXIF orientation tag to 1 (normal) after image rotation.
fn patch_orientation_in_place(full_payload: &mut Vec<u8>) {
    if !full_payload.starts_with(b"Exif\0\0") || full_payload.len() < 18 {
        return;
    }
    let tiff_data = &mut full_payload[6..];
    let is_le = tiff_data.starts_with(b"II");
    let offset = if is_le {
        u32::from_le_bytes([tiff_data[4], tiff_data[5], tiff_data[6], tiff_data[7]])
    } else {
        u32::from_be_bytes([tiff_data[4], tiff_data[5], tiff_data[6], tiff_data[7]])
    } as usize;

    if offset + 2 > tiff_data.len() {
        return;
    }
    let num_entries = if is_le {
        u16::from_le_bytes([tiff_data[offset], tiff_data[offset + 1]])
    } else {
        u16::from_be_bytes([tiff_data[offset], tiff_data[offset + 1]])
    } as usize;

    let mut pos = offset + 2;
    for _ in 0..num_entries {
        if pos + 12 > tiff_data.len() {
            break;
        }
        let tag = if is_le {
            u16::from_le_bytes([tiff_data[pos], tiff_data[pos + 1]])
        } else {
            u16::from_be_bytes([tiff_data[pos], tiff_data[pos + 1]])
        };
        if tag == 0x0112 {
            let val = pos + 8;
            if is_le {
                tiff_data[val] = 1;
                tiff_data[val + 1] = 0;
            } else {
                tiff_data[val] = 0;
                tiff_data[val + 1] = 1;
            }
            break;
        }
        pos += 12;
    }
}

/// Extracts ICC color profile from JPEG, PNG, or WebP files.
fn extract_icc_profile(input_path: &PathBuf) -> Option<Vec<u8>> {
    let file_bytes = std::fs::read(input_path).ok()?;
    let bytes_owned = bytes::Bytes::from(file_bytes);

    if let Ok(jpeg) = img_parts::jpeg::Jpeg::from_bytes(bytes_owned.clone()) {
        if let Some(icc) = jpeg.icc_profile() {
            return Some(icc.to_vec());
        }
    }
    if let Ok(decoder) = png::Decoder::new(std::fs::File::open(input_path).ok()?).read_info() {
        if let Some(icc) = decoder.info().icc_profile.clone() {
            return Some(icc.into_owned());
        }
    }
    if let Ok(webp) = img_parts::webp::WebP::from_bytes(bytes_owned) {
        if let Some(icc) = webp.icc_profile() {
            return Some(icc.to_vec());
        }
    }
    None
}

/// Applies EXIF orientation transform to correct image rotation.
fn apply_orientation(img: DynamicImage, path: &PathBuf) -> DynamicImage {
    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return img,
    };
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let orientation = exifreader
        .read_from_container(&mut bufreader)
        .ok()
        .and_then(|e| {
            e.get_field(exif::Tag::Orientation, exif::In::PRIMARY)
                .and_then(|f| f.value.get_uint(0))
        })
        .unwrap_or(1);

    match orientation {
        2 => img.fliph(),
        3 => img.rotate180(),
        4 => img.flipv(),
        5 => img.rotate90().fliph(),
        6 => img.rotate90(),
        7 => img.rotate270().fliph(),
        8 => img.rotate270(),
        _ => img,
    }
}

/// Converts image colors from input ICC profile to sRGB.
fn apply_color_correction(img: &mut DynamicImage, input_profile: &[u8]) -> Result<()> {
    let in_prof = lcms2::Profile::new_icc(input_profile).context("Invalid ICC profile")?;
    let out_prof = lcms2::Profile::new_srgb();
    let (fmt, _) = match img {
        DynamicImage::ImageRgb8(_) => (lcms2::PixelFormat::RGB_8, false),
        DynamicImage::ImageRgba8(_) => (lcms2::PixelFormat::RGBA_8, true),
        _ => return Ok(()),
    };
    let transform = lcms2::Transform::new(&in_prof, fmt, &out_prof, fmt, lcms2::Intent::Perceptual)
        .context("CMS")?;
    match img {
        DynamicImage::ImageRgb8(buffer) => transform.transform_in_place(buffer),
        DynamicImage::ImageRgba8(buffer) => transform.transform_in_place(buffer),
        _ => {}
    }
    Ok(())
}

/// High-quality image resizing using CatmullRom interpolation.
fn resize_image_fast(img: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
    use fast_image_resize as fr;
    let src = fr::images::Image::from_vec_u8(
        img.width(),
        img.height(),
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )?;
    let mut dst = fr::images::Image::new(width, height, fr::PixelType::U8x4);
    fr::Resizer::new().resize(
        &src,
        &mut dst,
        &fr::ResizeOptions::new()
            .resize_alg(fr::ResizeAlg::Convolution(fr::FilterType::CatmullRom)),
    )?;
    Ok(DynamicImage::ImageRgba8(
        image::ImageBuffer::from_raw(width, height, dst.into_vec()).context("Buffer")?,
    ))
}

/// Encodes image to JPEG format with mozjpeg compression and optional metadata.
fn encode_jpeg(
    img: &DynamicImage,
    quality: u8,
    metadata: Option<&Metadata>,
    writer: &mut BufWriter<File>,
) -> Result<()> {
    let rgb = img.to_rgb8();
    let (width, height) = (rgb.width() as usize, rgb.height() as usize);

    let buf = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
        comp.set_size(width, height);
        comp.set_quality((quality as f32).min(99.0));
        comp.set_progressive_mode();
        comp.set_optimize_scans(true);
        comp.set_smoothing_factor(1);
        comp.set_chroma_sampling_pixel_sizes((1, 1), (1, 1));

        let mut comp = comp.start_compress(Vec::new()).unwrap();
        comp.write_scanlines(rgb.as_raw()).unwrap();
        comp.finish().unwrap()
    }))
    .unwrap_or_else(|_| {
        let mut buf = Vec::new();
        let mut enc = jpeg_encoder::Encoder::new(&mut buf, quality);
        enc.set_optimized_huffman_tables(true);
        enc.set_progressive(true);
        let _ = enc.encode(
            rgb.as_raw(),
            rgb.width() as u16,
            rgb.height() as u16,
            jpeg_encoder::ColorType::Rgb,
        );
        buf
    });

    match img_parts::jpeg::Jpeg::from_bytes(buf.clone().into()) {
        Ok(mut jpeg) => {
            jpeg.set_icc_profile(Some(SRGB_ICC.into()));
            if let Some(meta) = metadata {
                if let Some(raw_exif) = &meta.exif {
                    let mut payload = raw_exif.clone();
                    patch_orientation_in_place(&mut payload);
                    let segments = jpeg.segments_mut();
                    segments.retain(|s| !s.contents().starts_with(b"Exif\0\0"));
                    segments.insert(
                        0,
                        JpegSegment::new_with_contents(0xE1, bytes::Bytes::from(payload)),
                    );
                }
            }
            jpeg.encoder().write_to(writer)?;
        }
        Err(_) => writer.write_all(&buf)?,
    }
    Ok(())
}

/// Encodes image to PNG format with optional oxipng optimization.
fn encode_png(img: &DynamicImage, compressed: bool, writer: &mut BufWriter<File>) -> Result<()> {
    let (width, height) = (img.width(), img.height());
    let has_alpha = img.color().has_alpha() && img.to_rgba8().pixels().any(|p| p.0[3] < 255);

    let mut buffer = Vec::new();
    {
        let (comp, filter) = if compressed {
            (png::Compression::Best, png::FilterType::Paeth)
        } else {
            (png::Compression::Default, png::FilterType::Sub)
        };

        let mut enc = png::Encoder::new(&mut buffer, width, height);
        enc.set_depth(png::BitDepth::Eight);
        enc.set_compression(comp);
        enc.set_filter(filter);
        enc.set_source_srgb(png::SrgbRenderingIntent::Perceptual);

        if has_alpha {
            enc.set_color(png::ColorType::Rgba);
            enc.write_header()?.write_image_data(&img.to_rgba8())?;
        } else {
            enc.set_color(png::ColorType::Rgb);
            enc.write_header()?.write_image_data(&img.to_rgb8())?;
        }
    }

    if compressed {
        let mut opts = oxipng::Options::from_preset(6);
        opts.strip = oxipng::StripChunks::Safe;
        opts.optimize_alpha = true;
        match oxipng::optimize_from_memory(&buffer, &opts) {
            Ok(optimized) => writer.write_all(&optimized)?,
            Err(_) => writer.write_all(&buffer)?,
        }
    } else {
        writer.write_all(&buffer)?;
    }
    Ok(())
}

/// Encodes image to WebP format with lossy compression.
fn encode_webp(img: &DynamicImage, quality: u8, writer: &mut BufWriter<File>) -> Result<()> {
    let rgba = img.to_rgba8();
    let (width, height) = (rgba.width(), rgba.height());
    let encoder = webp::Encoder::from_rgba(rgba.as_raw(), width, height);
    let webp_data = encoder.encode(quality as f32);

    match img_parts::webp::WebP::from_bytes(webp_data.to_vec().into()) {
        Ok(mut webp) => {
            webp.set_icc_profile(Some(SRGB_ICC.into()));
            webp.encoder().write_to(writer)?;
        }
        Err(_) => writer.write_all(&webp_data)?,
    }
    Ok(())
}

/// Main conversion function that orchestrates loading, processing, and encoding.
pub fn convert_image(input_path: &PathBuf, options: &ConversionOptions) -> Result<()> {
    validate_file_magic(input_path)?;

    const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;
    let file_size = std::fs::metadata(input_path).map(|m| m.len()).unwrap_or(0);
    if file_size > MAX_FILE_SIZE {
        anyhow::bail!("File too large (max 100MB)");
    }

    let ext = input_path
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();

    let (mut img, _) = if ext == "heic" || ext == "heif" {
        crate::heic::load_heic_via_libheif(input_path).context("Failed to load HEIC")?
    } else {
        (
            image::open(input_path).context("Failed to decode image")?,
            None,
        )
    };

    if ext != "heic" && ext != "heif" {
        img = apply_orientation(img, input_path);
    }

    let is_jpg_input = ext == "jpg" || ext == "jpeg";
    let is_jpg_output = matches!(options.format, ImageFormat::Jpeg);
    let metadata = if options.keep_metadata && is_jpg_input && is_jpg_output {
        extract_metadata(input_path)
    } else {
        None
    };

    if let Some(icc) = extract_icc_profile(input_path) {
        if img.color().has_alpha() {
            img = DynamicImage::ImageRgba8(img.to_rgba8());
        } else {
            img = DynamicImage::ImageRgb8(img.to_rgb8());
        }
        let _ = apply_color_correction(&mut img, &icc);
    }

    let processed = if options.resize {
        let (w, h) = (
            options.target_width.parse().unwrap_or(0),
            options.target_height.parse().unwrap_or(0),
        );
        if w > 0 || h > 0 {
            let (fw, fh) = (
                if w == 0 { u32::MAX } else { w },
                if h == 0 { u32::MAX } else { h },
            );
            resize_image_fast(&img, fw, fh)
                .unwrap_or_else(|_| img.resize(fw, fh, FilterType::Lanczos3))
        } else {
            img
        }
    } else {
        img
    };

    let parent = input_path.parent().unwrap_or(std::path::Path::new("."));
    let out_parent = if options.use_custom_output {
        options
            .custom_output_path
            .as_ref()
            .filter(|p| p.exists())
            .map(|p| p.as_path())
            .unwrap_or(parent)
    } else {
        parent
    };

    let mut stem = input_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    if !options.find_pattern.is_empty() {
        stem = stem.replace(&options.find_pattern, &options.replace_with);
    }
    if options.auto_suffix {
        stem.push_str(&get_smart_suffix(
            processed.width(),
            processed.height(),
            options.quality,
            options.format,
        ));
    }

    let ext_out = match options.format {
        ImageFormat::Jpeg => "jpg",
        ImageFormat::Png => "png",
        ImageFormat::WebP => "webp",
    };
    let output_path = out_parent.join(format!("{}{}.{}", options.prefix, stem, ext_out));

    let mut writer = BufWriter::new(File::create(&output_path)?);

    match options.format {
        ImageFormat::Jpeg => {
            encode_jpeg(&processed, options.quality, metadata.as_ref(), &mut writer)?
        }
        ImageFormat::Png => encode_png(&processed, options.png_compressed, &mut writer)?,
        ImageFormat::WebP => encode_webp(&processed, options.quality, &mut writer)?,
    }
    Ok(())
}

/// Generates resolution and quality suffix for filenames.
fn get_smart_suffix(width: u32, height: u32, quality: u8, format: ImageFormat) -> String {
    let short_side = width.min(height);
    if matches!(format, ImageFormat::Png) {
        format!("-{}p", short_side)
    } else {
        format!("-{}p-{}q", short_side, quality)
    }
}
