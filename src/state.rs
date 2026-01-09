//! Application state and data structures for conversion options and file management.

use std::collections::HashSet;
use std::path::PathBuf;

/// Supported output image formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    Png,
    WebP,
}

impl ImageFormat {
    pub const ALL: [ImageFormat; 3] = [ImageFormat::Jpeg, ImageFormat::Png, ImageFormat::WebP];
}

impl Default for ImageFormat {
    fn default() -> Self {
        ImageFormat::Jpeg
    }
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageFormat::Jpeg => "JPG",
                ImageFormat::Png => "PNG",
                ImageFormat::WebP => "WebP",
            }
        )
    }
}

/// Represents a file in the conversion queue.
#[derive(Debug, Clone)]
pub struct FileItem {
    pub id: uuid::Uuid,
    pub path: PathBuf,
    pub status: FileStatus,
}

impl FileItem {
    /// Creates new file item with pending status.
    pub fn new(path: PathBuf) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            path,
            status: FileStatus::Pending,
        }
    }
}

/// Processing status of a file item.
#[derive(Debug, Clone)]
pub enum FileStatus {
    Pending,
    Processing,
    Done,
    Error(String),
}

/// User-configurable conversion options.
#[derive(Debug, Clone)]
pub struct ConversionOptions {
    pub format: ImageFormat,
    pub quality: u8,
    pub png_compressed: bool,
    pub resize: bool,
    pub target_width: String,
    pub target_height: String,
    pub prefix: String,
    pub find_pattern: String,
    pub replace_with: String,
    pub auto_suffix: bool,
    pub use_custom_output: bool,
    pub custom_output_path: Option<PathBuf>,
    pub keep_metadata: bool,
    pub generate_log: bool,
    pub add_numbering: bool,
    pub is_dark_mode: bool,
    pub max_batch_size: usize,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            format: ImageFormat::Jpeg,
            quality: 80,
            png_compressed: true,
            resize: false,
            target_width: String::new(),
            target_height: String::new(),
            prefix: String::new(),
            find_pattern: String::new(),
            replace_with: String::new(),
            auto_suffix: false,
            use_custom_output: false,
            custom_output_path: None,
            keep_metadata: false,
            generate_log: false,
            add_numbering: false,
            is_dark_mode: false,
            max_batch_size: 50,
        }
    }
}

/// Main application state container.
pub struct AppState {
    pub files: Vec<FileItem>,
    pub selected_indices: HashSet<usize>,
    pub is_processing: bool,
    pub options: ConversionOptions,
    pub dragging_index: Option<usize>,
    pub hovered_index: Option<usize>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            selected_indices: HashSet::new(),
            is_processing: false,
            options: ConversionOptions::default(),
            dragging_index: None,
            hovered_index: None,
        }
    }
}
