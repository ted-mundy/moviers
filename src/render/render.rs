use std::sync::Arc;

use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;

use crate::clip::clip::VideoClip;
use std::process::{Command, Stdio};
use std::io::{Write, Read};

#[derive(derive_builder::Builder)]
pub struct ClipRenderer {
  pub clips: Vec<Arc<dyn VideoClip>>,
  pub output_path: String,
}

impl ClipRenderer {
  pub fn push_clip(&mut self, clip: Arc<dyn VideoClip>) {
    self.clips.push(clip);
  }

  pub fn write_video(&self) {
    // get ffmpeg to render the video
    let mut ffmpeg = Command::new("ffmpeg");
      ffmpeg.arg("-y")
      .arg("-loglevel")
      .arg("error")
      .arg("-f")
      .arg("rawvideo")
      .arg("-vcodec")
      .arg("rawvideo")
      .arg("-s")
      .arg("1920x1080")
      .arg("-pix_fmt")
      .arg("rgb24")
      .arg("-r")
      .arg("30")
      .arg("-an")
      .arg("-i")
      .arg("-")
      .arg("-vcodec")
      .arg("libx264")
      .arg("-preset")
      .arg("ultrafast")
      .arg("-pix_fmt")
      .arg("yuv420p")
      .arg(&self.output_path)
      .stdin(Stdio::piped());

    let mut ffmpeg = ffmpeg.spawn().unwrap();

    if let Some(stdin) = ffmpeg.stdin.as_mut() {

      let frame_size = 1920 * 1080 * 3; // width * height * channels (RGB)
      let mut frame = vec![0u8; frame_size];

      for _ in 0..10 * 30 {
        let r = 0;
        let g = 0;
        let b = 255;

        // for pixel in frame.chunks_exact_mut(3) {
        frame.par_chunks_exact_mut(3).for_each(|pixel| {
          pixel[0] = r;
          pixel[1] = g;
          pixel[2] = b;
        });

        stdin.write_all(&frame).expect("Failed to write to stdin");
      }

      stdin.flush().expect("Failed to flush stdin");

      ffmpeg.wait_with_output().unwrap();
    }
  }
}
