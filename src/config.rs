use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QualityPreset {
    Performance,
    Balanced,
    Quality,
    Ultra,
}

impl QualityPreset {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            QualityPreset::Performance => (80, 30),
            QualityPreset::Balanced => (120, 45),
            QualityPreset::Quality => (160, 60),
            QualityPreset::Ultra => (200, 75),
        }
    }

    pub fn next(&self) -> Self {
        match self {
            QualityPreset::Performance => QualityPreset::Balanced,
            QualityPreset::Balanced => QualityPreset::Quality,
            QualityPreset::Quality => QualityPreset::Ultra,
            QualityPreset::Ultra => QualityPreset::Performance,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RenderMode {
    ASCII,
    Block,
    Braille,
}

impl RenderMode {
    pub fn next(&self) -> Self {
        match self {
            RenderMode::ASCII => RenderMode::Block,
            RenderMode::Block => RenderMode::Braille,
            RenderMode::Braille => RenderMode::ASCII,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorMode {
    Monochrome,
    ANSI16,
    ANSI256,
    Truecolor,
}

impl ColorMode {
    pub fn next(&self) -> Self {
        match self {
            ColorMode::Monochrome => ColorMode::ANSI16,
            ColorMode::ANSI16 => ColorMode::ANSI256,
            ColorMode::ANSI256 => ColorMode::Truecolor,
            ColorMode::Truecolor => ColorMode::Monochrome,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub quality_preset: QualityPreset,
    pub render_mode: RenderMode,
    pub color_mode: ColorMode,
    pub hardware_decode: bool,
    pub dithering: bool,
    pub fps_limit: Option<u32>,
    pub show_fps: bool,
    pub volume: f32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            quality_preset: QualityPreset::Balanced,
            render_mode: RenderMode::Braille,
            color_mode: ColorMode::Truecolor,
            hardware_decode: true,
            dithering: true,
            fps_limit: None,
            show_fps: false,
            volume: 1.0,
        }
    }
}

impl Config {
    pub fn load_from_file(_path: &PathBuf) -> Result<Self, anyhow::Error> {
        Ok(Config::default())
    }

    pub fn save_to_file(&self, _path: &PathBuf) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
