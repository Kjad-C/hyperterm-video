use crate::config::ColorMode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RgbColor(pub u8, pub u8, pub u8);

impl RgbColor {
    pub fn to_grayscale(&self) -> u8 {
        ((self.0 as u32 * 299 + self.1 as u32 * 587 + self.2 as u32 * 114) / 1000) as u8
    }

    pub fn to_ansi16(&self) -> u8 {
        let gray = self.to_grayscale();
        if gray < 128 {
            0
        } else {
            7
        }
    }

    pub fn to_ansi256(&self) -> u8 {
        if self.0 == self.1 && self.1 == self.2 {
            let gray = self.0;
            if gray < 48 {
                16
            } else if gray < 115 {
                let index = (gray - 48) / 10;
                232 + index
            } else {
                let index = (gray - 115) / 10;
                232 + 24 + index
            }
        } else {
            let r = (self.0 as f32 / 255.0 * 5.0) as u8;
            let g = (self.1 as f32 / 255.0 * 5.0) as u8;
            let b = (self.2 as f32 / 255.0 * 5.0) as u8;
            16 + 36 * r + 6 * g + b
        }
    }
}

pub struct AnsiRenderer {
    color_mode: ColorMode,
    width: u32,
    height: u32,
}

impl AnsiRenderer {
    pub fn new(color_mode: ColorMode, width: u32, height: u32) -> Self {
        AnsiRenderer {
            color_mode,
            width,
            height,
        }
    }

    pub fn set_color(&self, color: RgbColor) -> String {
        match self.color_mode {
            ColorMode::Monochrome => String::new(),
            ColorMode::ANSI16 => {
                let ansi_code = color.to_ansi16();
                format!("\x1b[{}m", if ansi_code == 0 { 30 } else { 37 })
            }
            ColorMode::ANSI256 => {
                let ansi_code = color.to_ansi256();
                format!("\x1b[38;5;{}m", ansi_code)
            }
            ColorMode::Truecolor => {
                format!("\x1b[38;2;{};{};{}m", color.0, color.1, color.2)
            }
        }
    }

    pub fn set_bg_color(&self, color: RgbColor) -> String {
        match self.color_mode {
            ColorMode::Monochrome => String::new(),
            ColorMode::ANSI16 => {
                let ansi_code = color.to_ansi16();
                format!("\x1b[{}m", if ansi_code == 0 { 40 } else { 47 })
            }
            ColorMode::ANSI256 => {
                let ansi_code = color.to_ansi256();
                format!("\x1b[48;5;{}m", ansi_code)
            }
            ColorMode::Truecolor => {
                format!("\x1b[48;2;{};{};{}m", color.0, color.1, color.2)
            }
        }
    }

    pub fn reset() -> &'static str {
        "\x1b[0m"
    }
}

pub fn floyd_steinberg_dither(image: &mut [u8], width: u32, height: u32, max_value: u8) {
    let width = width as usize;
    let height = height as usize;

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let old_value = image[idx];
            let new_value = if old_value < max_value / 2 { 0 } else { max_value };
            let error = old_value as i16 - new_value as i16;
            image[idx] = new_value;

            if x + 1 < width {
                let right_idx = y * width + x + 1;
                let right = image[right_idx] as i16 + (error * 7) / 16;
                image[right_idx] = right.clamp(0, 255) as u8;
            }

            if y + 1 < height {
                if x > 0 {
                    let down_left_idx = (y + 1) * width + x - 1;
                    let down_left = image[down_left_idx] as i16 + (error * 3) / 16;
                    image[down_left_idx] = down_left.clamp(0, 255) as u8;
                }

                let down_idx = (y + 1) * width + x;
                let down = image[down_idx] as i16 + (error * 5) / 16;
                image[down_idx] = down.clamp(0, 255) as u8;

                if x + 1 < width {
                    let down_right_idx = (y + 1) * width + x + 1;
                    let down_right = image[down_right_idx] as i16 + error / 16;
                    image[down_right_idx] = down_right.clamp(0, 255) as u8;
                }
            }
        }
    }
}
