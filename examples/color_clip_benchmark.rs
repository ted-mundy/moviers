use std::{process::Command, sync::Arc};

use moviers::{clip::color::{Color, ColorClip}, render::render::ClipRenderer};

struct RenderInput {
  width: u32,
  height: u32,
  fps: u32,
  duration: u32,
  filename: String,
}

fn render_color_clip() -> std::time::Duration {
  let start = std::time::Instant::now();

  let color_clip = ColorClip {
    color: Color::RGB(0, 0, 255),
    width: 1920,
    height: 1080,
    duration: 10.0,
    fps: 30,
  };

  const OUTPUT_FILENAME: &str = "color_clip_rust.mp4";

  let clip_renderer = ClipRenderer {
    clips: vec![Arc::new(color_clip)],
    output_path: OUTPUT_FILENAME.to_string(),
  };

  clip_renderer.write_video();

  start.elapsed()
}

fn render_color_clip_ffmpeg(input: &RenderInput) -> std::time::Duration {
  let start = std::time::Instant::now();

  let ffmpeg_output = Command::new("ffmpeg").args(
    &[
      "-y",
      "-f", "lavfi",
      "-i", &format!("color=c=blue:s={}x{}", input.width, input.height),
      "-t", &format!("{}", input.duration),
      "-r", &format!("{}", input.fps),
      "-c:v", "libx264",
      "-pix_fmt", "yuv420p",
      &input.filename,
    ]
  ).output().expect("Failed to execute ffmpeg command");

  if !ffmpeg_output.status.success() {
    panic!("Failed to render color clip with ffmpeg: {}", String::from_utf8_lossy(&ffmpeg_output.stderr));
  }

  println!("Rendered color clip with ffmpeg");

  start.elapsed()
}

fn main() {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;
    const FPS: u32 = 30;
    const DURATION: u32 = 10;

    println!("Running single-run benchmark comparison...\n");

    println!("Testing FFMPEG direct implementation...");
    let ffmpeg_time = render_color_clip_ffmpeg(&RenderInput {
      width: WIDTH,
      height: HEIGHT,
      fps: FPS,
      duration: DURATION,
      filename: "color_clip_ffmpeg.mp4".to_string(),
    });
    println!("FFMPEG took: {:?}\n", ffmpeg_time);

    println!("Testing Rust implementation...");
    let rust_time = render_color_clip();
    println!("Rust took: {:?}\n", rust_time);

    println!("Results comparison:");
    println!("------------------");
    println!("FFMPEG: {:?}", ffmpeg_time);
    println!("Rust:   {:?}", rust_time);

    let ratio = rust_time.as_secs_f64() / ffmpeg_time.as_secs_f64();
    println!("Ratio (Rust/FFMPEG): {:.2}x", ratio);

    println!("\nDeleting output files...");
    std::fs::remove_file("color_clip_rust.mp4").expect("Failed to delete Rust output file");
    std::fs::remove_file("color_clip_ffmpeg.mp4").expect("Failed to delete FFMPEG output file");
}
