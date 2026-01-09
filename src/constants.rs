//! Application constants for EXIF handling, file validation, and UI sizing.
#![allow(dead_code)]

pub const EXIF_APP1_MARKER: u8 = 0xE1;
pub const EXIF_ORIENTATION_TAG: u16 = 0x0112;
pub const EXIF_HEADER: &[u8] = b"Exif\0\0";
pub const MAX_FILE_SIZE_BYTES: u64 = 100 * 1024 * 1024;
pub const SUPPORTED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "webp", "heic", "heif"];
pub const TEXT_SIZE_SMALL: u16 = 12;
pub const TEXT_SIZE_NORMAL: u16 = 14;
pub const TEXT_SIZE_TITLE: u16 = 18;
