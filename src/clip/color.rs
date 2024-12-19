use super::clip::{ClipError, VideoClip};

pub enum Color {
  RGB(u8, u8, u8),  // 255, 0, 0
  Hex(HexColor),    // #FF0000 or 0xFF0000
  Name(String),     // red, blue, green
}

pub enum HexColor {
  String(String), // #FF0000
  Number(u32),    // 0xFF0000
}

pub struct ColorClip {
  pub color: Color,
  pub width: u32,
  pub height: u32,
  pub duration: f64,
  pub fps: u32,
}

impl VideoClip for ColorClip {
  fn get_frame(&self, _: usize) -> Result<[u8; 3], ClipError> {
    match &self.color {
      Color::RGB(r, g, b) => Ok([*r, *g, *b]),
      Color::Hex(hex_data) => {
        match hex_data {
          HexColor::String(hex) => {
            let hex = hex.trim_start_matches("#");
            let hex = u32::from_str_radix(hex, 16).map_err(|_| ClipError::InvalidData)?;

            hex_col_to_slice(hex)
          },
          HexColor::Number(hex) => hex_col_to_slice(*hex)
        }
      },
      Color::Name(name) => {
        match name.as_str() {
          "red" => Ok([255, 0, 0]),
          "green" => Ok([0, 255, 0]),
          "blue" => Ok([0, 0, 255]),
          // TODO
          _ => Err(ClipError::InvalidData),
        }
      }
    }
  }

  fn get_duration(&self) -> f64 {
    self.duration
  }

  fn get_fps(&self) -> u32 {
    self.fps
  }
}

fn hex_col_to_slice(hex: u32) -> Result<[u8; 3], ClipError> {
  let r = (hex >> 16) & 0xFF;
  let g = (hex >> 8) & 0xFF;
  let b = hex & 0xFF;

  Ok([r as u8, g as u8, b as u8])
}
