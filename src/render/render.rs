use std::borrow::Borrow;
use std::env;
use std::sync::Arc;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use thiserror::Error;

use crate::clip::clip::VideoClip;
use std::process::{Command, Stdio};
use std::io::Write;

#[derive(derive_builder::Builder)]
pub struct ClipRenderer {
  pub clips: Vec<Arc<dyn VideoClip>>,
  pub output_path: String,
}

#[derive(Error, Debug)]
pub enum RenderError {
  #[error("No clips to render")]
  NoClips,
}

impl ClipRenderer {
  pub fn write_video(&self) {
    if self.clips.is_empty() {
      return
    }

    let ffmpeg_binary: String = env::var("FFMPEG_BINARY").unwrap_or("ffmpeg".to_string()); // support custom ffmpeg binary path

    let mut ffmpeg_process = Command::new(ffmpeg_binary);

    let [canvas_width, canvas_height] = self.get_canvas_size().unwrap();
    let fps = self.get_base_fps().to_string();

    // default args
    ffmpeg_process.arg("-y")
      .arg("-loglevel")
      .arg("error")
      .arg("-f")
      .arg("rawvideo")
      .arg("-vcodec")
      .arg("rawvideo")
      .arg("-s")
      .arg(format!("{}x{}", canvas_width, canvas_height))
      .arg("-pix_fmt")
      .arg("rgb24")
      .arg("-r")
      .arg(fps)
      .arg("-i")
      .arg("-")
      .arg("-vcodec")
      .arg("libx264")
      .arg(&self.output_path)
      .stdin(Stdio::piped());

    let mut ffmpeg = ffmpeg_process.spawn().unwrap();

    if let Some(stdin) = ffmpeg.stdin.as_mut() {
      let frame_size = (canvas_width * canvas_height * 3) as usize; // we don't use alpha atm
      let frame = vec![0u8; frame_size];

      // TODO: for now, we don't allow overlapping clips. when we do, this logic will need to be tweaked
      for clip in self.clips.iter() {
        let clip_duration = clip.get_duration();
        let clip_fps = clip.get_fps().unwrap_or(30);
        let clip_frame_count = (clip_duration * clip_fps as f64).ceil() as usize;

        for frame_number in 0..clip_frame_count {
          let frame_data = clip.get_frame(frame_number).unwrap();

          let mut frame_copy = frame.clone();
          frame_copy.par_chunks_exact_mut(3).for_each(|pixel| {
            pixel[0] = frame_data[0];
            pixel[1] = frame_data[1];
            pixel[2] = frame_data[2];
          });

          stdin.write_all(&frame_copy).expect("Failed to write to stdin");
        }
      }

      stdin.flush().expect("Failed to flush stdin");

      ffmpeg.wait_with_output().unwrap();
    }
  }

  fn get_canvas_size(&self) -> Result<[u32; 2], RenderError> {
    if self.clips.is_empty() {
      return Err(RenderError::NoClips);
    }

    Ok(self.clips[0].get_size())
  }

  fn get_base_fps(&self) -> u32 {
    if self.clips.is_empty() {
      return 30;
    }

    self.clips[0].get_fps().unwrap_or(30)
  }
}
