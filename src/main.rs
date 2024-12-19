use std::sync::Arc;

use moviers::{clip::color, render::render};

fn main() {
  let clip = color::ColorClip {
    color: color::Color::RGB(255, 0, 0),
    width: 1920,
    height: 1080,
    duration: 1.0
  };

  let clip2 = color::ColorClip {
    color: color::Color::RGB(0, 255, 0),
    width: 1920,
    height: 1080,
    duration: 1.0
  };

  let clip3 = color::ColorClip {
    color: color::Color::RGB(0, 0, 255),
    width: 1920,
    height: 1080,
    duration: 2.0
  };

  let renderer = render::ClipRenderer {
    output_path: String::from("output.mp4"),
    clips: vec![Arc::new(clip), Arc::new(clip2), Arc::new(clip3)]
  };

  renderer.write_video();
}
