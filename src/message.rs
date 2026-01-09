//! Application message types for UI events and state updates.

use crate::state::ImageFormat;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    WindowResized,
    AddFilesClicked,
    FilesSelected(Vec<PathBuf>),
    ExternalFilesDropped(Vec<PathBuf>),
    ItemDragStarted(usize),
    ItemDropped,
    ItemHovered(Option<usize>),
    FormatSelected(ImageFormat),
    QualityChanged(u8),
    QualityInputChanged(String),
    PngCompressionToggled(bool),
    ResizeToggled(bool),
    WidthChanged(String),
    HeightChanged(String),
    PrefixChanged(String),
    FindPatternChanged(String),
    ReplaceWithChanged(String),
    AutoSuffixToggled(bool),
    ToggleSelection(usize),
    DeleteSelected,
    ClearList,
    ToggleCustomOutput(bool),
    BrowseOutputClicked,
    OutputFolderSelected(Option<PathBuf>),
    ToggleKeepMetadata(bool),
    ToggleGenerateLog(bool),
    AddNumberingToggled(bool),
    ManualGenerateLogClicked,
    DarkThemeToggled(bool),
    ConvertClicked,
    OverwriteDecision(bool),
    FileConverted(uuid::Uuid, Result<(), String>),
    ConversionFinished,
}
