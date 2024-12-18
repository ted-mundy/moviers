use std::sync::Arc;

use moviers::clip::color as color;
use moviers::render::render as render;


#[test]
fn test_clip_creation() {
  let clip = color::ColorClip {
    color: color::Color::RGB(255, 0, 0),
    width: 1920,
    height: 1080,
    duration: 30,
  };

  let mut renderer = render::ClipRenderer {
    output_path: String::from("output.mp4"),
    clips: Vec::new(),
  };

  renderer.push_clip(Arc::new(clip));

  assert_eq!(renderer.clips.len(), 1);

  renderer.write_video();
}
