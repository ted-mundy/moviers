use std::sync::Arc;

use moviers::clip::color as color;
use moviers::render::render as render;


#[test]
fn test_clip_creation() {
  const CLIP_FPS: u32 = 30;
  let clip = color::ColorClip {
    color: color::Color::RGB(255, 0, 0),
    width: 1920,
    height: 1080,
    duration: 0.3,
    fps: CLIP_FPS,
  };

  let mut renderer = render::ClipRenderer {
    output_path: String::from("output.mp4"),
    clips: Vec::new(),
    output_fps: None,
  };

  renderer.push_clip(Arc::new(clip));

  println!("Output FPS: {}", renderer.get_output_fps().unwrap());

  assert_eq!(renderer.clips.len(), 1);
  assert_eq!(renderer.get_output_fps().unwrap(), CLIP_FPS);

  renderer.write_video();
}
