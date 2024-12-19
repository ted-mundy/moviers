use std::env;
use std::sync::Arc;

use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::slice::{ParallelSlice, ParallelSliceMut};
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
      .arg("-pix_fmt")
      .arg("yuv420p")
      .arg(&self.output_path)
      .stdin(Stdio::piped());

    let mut ffmpeg = ffmpeg_process.spawn().unwrap();

    if let Some(stdin) = ffmpeg.stdin.as_mut() {
      let frame_size = (canvas_width * canvas_height * 3) as usize; // we don't use alpha atm
      let max_frames = self.clips.iter().map(|clip| {
        let clip_duration = clip.get_duration();
        let clip_fps = clip.get_fps().unwrap_or(30);
        let clip_start_time = clip.get_start_time();
        let clip_frame_count = (clip_duration * clip_fps as f64).ceil() as usize;

        clip_frame_count + (clip_start_time * clip_fps as f64).ceil() as usize
      }).max().unwrap();

      for frame_i in 0..max_frames {
        let mut frame_data = vec![0u8; frame_size];

        // TODO: some sort of z-index system for clips. for now, we will go in order
        for clip in self.clips.iter() {
          // should we even be rendering this clip?
          let clip_start_time = clip.get_start_time();
          let clip_duration = clip.get_duration();
          let clip_fps = clip.get_fps().unwrap_or(30);
          let clip_frame_count = (clip_duration * clip_fps as f64).ceil() as usize;

          // if we are before the clip starts, skip
          if frame_i < (clip_start_time * clip_fps as f64).ceil() as usize {
            continue;
          }

          // if we are after the clip ends, skip
          if frame_i >= clip_frame_count + (clip_start_time * clip_fps as f64).ceil() as usize {
            continue;
          }

          let clip_frame_data = clip.get_frame(frame_i).unwrap();

          // now, add the clip frame data to the frame data
          for (i, pixel) in clip_frame_data.chunks_exact(3).enumerate() {
            let x = i % canvas_width as usize;
            let y = i / canvas_width as usize;

            let frame_index = (y * canvas_width as usize + x) * 3;

            frame_data[frame_index] = pixel[0];
            frame_data[frame_index + 1] = pixel[1];
            frame_data[frame_index + 2] = pixel[2];
          }
        }

        // if our frame data is less than the frame size, pad it with 0s
        if frame_data.len() < frame_size {
          frame_data.resize(frame_size, 0);
        }

        stdin.write_all(&frame_data).expect("Failed to write to stdin");
      }

      // for clip in self.clips.iter() {
      //   let clip_duration = clip.get_duration();
      //   let clip_fps = clip.get_fps().unwrap_or(30);
      //   let clip_frame_count = (clip_duration * clip_fps as f64).ceil() as usize;

      //   for frame_number in 0..clip_frame_count {
      //     let frame_data = clip.get_frame(frame_number).unwrap();

      //     stdin.write_all(&frame_data).expect("Failed to write to stdin");
      //   }
      // }

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
