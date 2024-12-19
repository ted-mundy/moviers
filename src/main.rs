use std::sync::Arc;

use moviers::{clip::{clip::VideoClip, color::{self, ColorClip}}, render::render};

fn main() {
  let clips: Vec<Arc<dyn VideoClip>> = vec![
    Arc::new(ColorClip {
      color: color::Color::RGB(255, 0, 0),
      width: 1920,
      height: 1080,
      duration: 3.0,
      start_time: 0.0,
      position: [0, 0],
    }),
    Arc::new(ColorClip {
      color: color::Color::RGB(0, 255, 0),
      width: 1920,
      height: 1080 / 3 * 2,
      duration: 2.0,
      start_time: 1.0,
      position: [0, 0],
    }),
    Arc::new(ColorClip {
      color: color::Color::RGB(0, 0, 255),
      width: 1920,
      height: 1080 / 3,
      duration: 1.0,
      start_time: 2.0,
      position: [0, 0],
    }),
    ];

  let renderer = render::ClipRenderer {
    output_path: String::from("output.mp4"),
    clips,
  };

  renderer.write_video();
}
