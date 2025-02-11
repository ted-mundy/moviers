use std::env;
use std::sync::Arc;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::slice::ParallelSlice;
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
  pub fn write_video(&self) -> Result<f64, RenderError> {
    if self.clips.is_empty() {
      return Err(RenderError::NoClips)
    }

    let start = std::time::Instant::now();

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

          let [clip_width, clip_height] = clip.get_size();
          let [clip_x, clip_y] = clip.get_position();

          // if we are out of bounds, skip
          if clip_x + (clip_width as i32) <= 0 || clip_x > (canvas_width as i32) {
            continue;
          }

          if clip_y + (clip_height as i32) <= 0 || clip_y > (canvas_height as i32) {
            continue;
          }

          // TODO:  handle videos with different fps. the clip has a higher fps, it's ok, just skip frames (as it should)
          //        if the clip has a lower fps, we should use the last frame of the clip until we reach the next valid frame
          let clip_frame_data = clip.get_frame(frame_i).unwrap();

          // now, add the clip frame data to the frame data
          let chunks = clip_frame_data.par_chunks_exact(3).collect::<Vec<_>>();
          for (i, pixel) in chunks.into_par_iter().collect::<Vec<_>>().iter().enumerate() {
            // if we are out of the X bounds, wrap around
            let x = (i % clip_width as usize) as i32; // x is the remainder of i divided by the clip width. this makes it so we can wrap around
            let y = (i / clip_width as usize) as i32; // ...and same idea for y

            let pixel_offset = ((clip_y + y) * canvas_width as i32 + (clip_x + x)) as usize * 3;

            if pixel_offset >= frame_data.len() {
              continue;
            }

            frame_data[pixel_offset] = pixel[0];
            frame_data[pixel_offset + 1] = pixel[1];
            frame_data[pixel_offset + 2] = pixel[2];
          }
        }

        // if our frame data is less than the frame size, pad it with 0s
        if frame_data.len() < frame_size {
          frame_data.resize(frame_size, 0);
        }

        stdin.write_all(&frame_data).expect("Failed to write to stdin");
      }

      stdin.flush().expect("Failed to flush stdin");

      ffmpeg.wait_with_output().unwrap();
    }

    Ok(start.elapsed().as_secs_f64())
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
