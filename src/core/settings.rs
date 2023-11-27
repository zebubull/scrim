use std::path::Path;

use color_eyre::eyre::{Context, Report, Result};
use ratatui::style::Color;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum SaveFormat {
    JSON,
    YAML,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum TermColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightGray,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl From<TermColor> for Color {
    fn from(value: TermColor) -> Self {
        match value {
            TermColor::Black => Color::Black,
            TermColor::Red => Color::Red,
            TermColor::Green => Color::Green,
            TermColor::Yellow => Color::Yellow,
            TermColor::Blue => Color::Blue,
            TermColor::Magenta => Color::Magenta,
            TermColor::Cyan => Color::Cyan,
            TermColor::Gray => Color::Gray,
            TermColor::DarkGray => Color::DarkGray,
            TermColor::LightRed => Color::LightRed,
            TermColor::LightGreen => Color::LightGreen,
            TermColor::LightYellow => Color::LightYellow,
            TermColor::LightBlue => Color::LightBlue,
            TermColor::LightMagenta => Color::LightMagenta,
            TermColor::LightCyan => Color::LightCyan,
            TermColor::LightGray => Color::White,
            TermColor::Rgb(r, g, b) => Color::Rgb(r, g, b),
            TermColor::Indexed(i) => Color::Indexed(i),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub format: SaveFormat,
    pub background: TermColor,
    pub foreground: TermColor,
    pub popup_background: TermColor,
    pub popup_foreground: TermColor,
    pub highlight: TermColor,
    pub tab_select: TermColor,
}

impl Settings {
    pub fn load_or_default(path: &Path) -> (Self, Option<Report>) {
        match Self::load(path) {
            Ok(s) => (s, None),
            Err(e) => (Settings::default(), Some(e)),
        }
    }

    fn load(path: &Path) -> Result<Self> {
        let data = std::fs::read(path).wrap_err_with(|| {
            format!(
                "failed to load settings from file '{}'",
                path.to_str().unwrap_or(&path.to_string_lossy())
            )
        })?;
        let settings = serde_yaml::from_slice(data.as_slice()).wrap_err_with(|| {
            format!(
                "failed to parse settings from file '{}'",
                path.to_str().unwrap_or(&path.to_string_lossy())
            )
        })?;
        Ok(settings)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            format: SaveFormat::YAML,
            background: TermColor::Yellow,
            foreground: TermColor::Yellow,
            popup_background: TermColor::Yellow,
            popup_foreground: TermColor::Black,
            highlight: TermColor::Green,
            tab_select: TermColor::LightGray,
        }
    }
}
