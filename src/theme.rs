//! Design system tokens: modern color palettes, spacing, and dimensions.
#![allow(dead_code)]
use iced::Color;

/// Light theme color palette.
pub mod colors {
    use super::*;
    pub const PRIMARY: Color = Color::from_rgb(0.35, 0.47, 0.98);
    pub const PRIMARY_HOVER: Color = Color::from_rgb(0.25, 0.37, 0.88);
    pub const ACCENT: Color = Color::from_rgb(0.56, 0.27, 0.98);
    pub const BACKGROUND: Color = Color::from_rgb(0.96, 0.97, 0.98);
    pub const SURFACE: Color = Color::WHITE;
    pub const SURFACE_ELEVATED: Color = Color::from_rgb(0.99, 0.99, 1.0);
    pub const BORDER: Color = Color::from_rgb(0.88, 0.90, 0.92);
    pub const TEXT: Color = Color::from_rgb(0.1, 0.1, 0.12);
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.4, 0.42, 0.48);
    pub const MUTED: Color = Color::from_rgb(0.55, 0.58, 0.65);
    pub const SUCCESS: Color = Color::from_rgb(0.16, 0.71, 0.45);
    pub const ERROR: Color = Color::from_rgb(0.91, 0.29, 0.33);
    pub const WARNING: Color = Color::from_rgb(0.96, 0.62, 0.12);
    pub const PROCESSING: Color = Color::from_rgb(0.35, 0.47, 0.98);
}

/// Dark theme color palette with modern aesthetic.
pub mod dark {
    use super::*;
    pub const PRIMARY: Color = Color::from_rgb(0.45, 0.56, 1.0);
    pub const PRIMARY_HOVER: Color = Color::from_rgb(0.55, 0.66, 1.0);
    pub const ACCENT: Color = Color::from_rgb(0.70, 0.45, 1.0);
    pub const BACKGROUND: Color = Color::from_rgb(0.08, 0.09, 0.11);
    pub const SURFACE: Color = Color::from_rgb(0.12, 0.13, 0.16);
    pub const SURFACE_ELEVATED: Color = Color::from_rgb(0.16, 0.17, 0.20);
    pub const BORDER: Color = Color::from_rgb(0.22, 0.24, 0.28);
    pub const TEXT: Color = Color::from_rgb(0.95, 0.96, 0.98);
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.68, 0.72, 0.78);
    pub const MUTED: Color = Color::from_rgb(0.50, 0.54, 0.62);
    pub const SUCCESS: Color = Color::from_rgb(0.30, 0.82, 0.55);
    pub const ERROR: Color = Color::from_rgb(1.0, 0.45, 0.48);
    pub const WARNING: Color = Color::from_rgb(1.0, 0.72, 0.25);
    pub const PROCESSING: Color = Color::from_rgb(0.45, 0.56, 1.0);
}

/// Spacing scale for consistent layout.
pub mod spacing {
    pub const XXS: u16 = 2;
    pub const XS: u16 = 4;
    pub const SM: u16 = 8;
    pub const MD: u16 = 12;
    pub const LG: u16 = 16;
    pub const XL: u16 = 24;
    pub const XXL: u16 = 32;
}

/// Component dimensions.
pub mod dimensions {
    pub const FILE_LIST_HEIGHT: f32 = 220.0;
    pub const CARD_RADIUS: f32 = 12.0;
    pub const BUTTON_RADIUS: f32 = 8.0;
    pub const INPUT_RADIUS: f32 = 8.0;
    pub const PROGRESS_BAR_HEIGHT: f32 = 6.0;
    pub const GROUP_PADDING: u16 = 16;
    pub const HEADER_HEIGHT: f32 = 48.0;
}

/// Font sizes for typography.
pub mod typography {
    pub const TITLE: u16 = 22;
    pub const HEADING: u16 = 16;
    pub const BODY: u16 = 14;
    pub const CAPTION: u16 = 12;
    pub const SMALL: u16 = 11;
}
