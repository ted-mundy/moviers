use super::clip::VideoClip;

pub enum Color {
  RGB(u8, u8, u8),  // 255, 0, 0
  Hex(String),      // #FF0000 // ? should this be a u32? maybe support both if possible
  Name(String),     // red, blue, green
}

pub struct ColorClip {
  pub color: Color,
  pub width: u32,
  pub height: u32,
  pub duration: f64,
  pub fps: u32,
}

impl VideoClip for ColorClip {
  fn get_frame(&self, _: usize) -> Result<(u8, u8, u8), String> {
    match self.color {
      Color::RGB(r, g, b) => {
        return Ok((r, g, b));
      }
      // match everything else. we will just error out for now.
      _ => {
        return Err(String::from("Color type not supported"));
      }
    }
  }

  fn get_duration(&self) -> f64 {
    return self.duration;
  }

  fn get_fps(&self) -> u32 {
    return self.fps;
  }
}