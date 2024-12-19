use moviers::clip::clip::VideoClip;
use moviers::clip::color as color;


#[test]
fn test_frame_rgb() {
  let mut clip = color::ColorClip {
    color: color::Color::RGB(255, 0, 0),
    width: 1920,
    height: 1080,
    duration: 30.0,
    start_time: 0.0,
    position: [0, 0],
  };

  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [255, 0, 0]);

  clip.color = color::Color::RGB(0, 255, 0);
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0, 255, 0]);

  clip.color = color::Color::RGB(0, 0, 255);
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0, 0, 255]);

  clip.color = color::Color::RGB(0xC0, 0xFF, 0xFE);
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0xC0, 0xFF, 0xFE]);
}


#[test]
fn test_frame_hex_str() {
  let mut clip = color::ColorClip {
    color: color::Color::Hex(color::HexColor::String(String::from("#FF0000"))),
    width: 1920,
    height: 1080,
    duration: 30.0,
    start_time: 0.0,
    position: [0, 0],
  };

  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [255, 0, 0]);

  clip.color = color::Color::Hex(color::HexColor::String(String::from("#00FF00")));
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0, 255, 0]);

  clip.color = color::Color::Hex(color::HexColor::String(String::from("#0000FF")));
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0, 0, 255]);

  clip.color = color::Color::Hex(color::HexColor::String(String::from("#C0FFFE")));
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0xC0, 0xFF, 0xFE]);

  clip.color = color::Color::Hex(color::HexColor::String(String::from("#NOWORK")));
  let frame = clip.get_frame(0);
  assert!(frame.is_err());
}

#[test]
fn test_frame_hex_u32() {
  let mut clip = color::ColorClip {
    color: color::Color::Hex(color::HexColor::Number(0xFF0000)),
    width: 1920,
    height: 1080,
    duration: 30.0,
    start_time: 0.0,
    position: [0, 0],
  };

  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [255, 0, 0]);

  clip.color = color::Color::Hex(color::HexColor::Number(0x00FF00));
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0, 255, 0]);

  clip.color = color::Color::Hex(color::HexColor::Number(0x0000FF));
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0, 0, 255]);

  clip.color = color::Color::Hex(color::HexColor::Number(0xC0FFFE));
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0xC0, 0xFF, 0xFE]);

  clip.color = color::Color::Hex(color::HexColor::Number(0xFFFFFF + 1));
  let frame = clip.get_frame(0);
  assert!(frame.is_err());
}

#[test]
fn test_frame_name() {
  let mut clip = color::ColorClip {
    color: color::Color::Name(color::ColorName::Red),
    width: 1920,
    height: 1080,
    duration: 30.0,
    start_time: 0.0,
    position: [0, 0],
  };

  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [255, 0, 0]);

  clip.color = color::Color::Name(color::ColorName::Green);
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0, 255, 0]);

  clip.color = color::Color::Name(color::ColorName::Blue);
  let frame = clip.get_frame(0);
  assert_eq!(frame.unwrap(), [0, 0, 255]);
}
