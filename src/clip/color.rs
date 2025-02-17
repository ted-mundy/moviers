use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};

use super::clip::{ClipError, VideoClip};

pub enum Color {
  RGB(u8, u8, u8),  // 255, 0, 0
  Hex(HexColor),    // #FF0000 or 0xFF0000
  Name(ColorName),  // red, blue, green
}

pub enum HexColor {
  String(String), // #FF0000
  Number(u32),    // 0xFF0000
}

pub enum ColorName {
  Red,
  Green,
  Blue,
}

pub struct ColorClip {
  pub color: Color,
  pub width: u32,
  pub height: u32,
  pub duration: f64,
  pub start_time: f64,
  pub position: [i32; 2],
}

impl VideoClip for ColorClip {
  fn get_frame(&self, _: usize) -> Result<Vec<u8>, ClipError> {
    let [width, height] = self.get_size();
    let frame_size = (width * height * 3) as usize;
    let mut frame = vec![0u8; frame_size];
    let [r, g, b] = get_color(&self.color)?;
    frame.par_chunks_exact_mut(3).for_each(|pixel| {
      pixel[0] = r;
      pixel[1] = g;
      pixel[2] = b;
    });

    Ok(frame)
  }

  fn get_duration(&self) -> f64 {
    self.duration
  }

  fn get_fps(&self) -> Result<u32, ClipError> {
    Ok(1)
  }

  fn get_size(&self) -> [u32; 2] {
    [self.width, self.height]
  }

  fn get_start_time(&self) -> f64 {
    self.start_time
  }

  fn get_position(&self) -> [i32; 2] {
    self.position
  }
}

fn hex_col_to_slice(hex: u32) -> Result<[u8; 3], ClipError> {
  if hex > 0xFFFFFF {
    return Err(ClipError::InvalidData)
  };
  let r = ((hex >> 16) & 0xFF) as u8;
  let g = ((hex >> 8) & 0xFF) as u8;
  let b = (hex & 0xFF) as u8;

  Ok([r, g, b])
}


fn get_color(color: &Color) -> Result<[u8; 3], ClipError> {
  match color {
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
      match name {
        ColorName::Red => Ok([255, 0, 0]),
        ColorName::Green => Ok([0, 255, 0]),
        ColorName::Blue => Ok([0, 0, 255]),
      }
    }
  }
}
