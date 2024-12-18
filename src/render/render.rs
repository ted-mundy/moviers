use std::sync::Arc;

use thiserror::Error;

use crate::clip::clip::{ VideoClip, VideoError };
use std::process::{Command, Stdio};
use std::io::{Write, Read};

#[derive(Debug, Error)]
pub enum RenderError {
  #[error("No clips to render")]
  NoClips,

  #[error("VideoError: {0}")]
  Video(#[from] VideoError),
}

#[derive(derive_builder::Builder)]
pub struct ClipRenderer {
  pub clips: Vec<Arc<dyn VideoClip>>,
  pub output_path: String,
  // we can set this manually if we want to override the fps of the clips
  // if we don't set this, we will use the highest fps of the clips
  pub output_fps: Option<u32>,
}

impl ClipRenderer {
  pub fn push_clip(&mut self, clip: Arc<dyn VideoClip>) {
    self.clips.push(clip);
  }

  pub fn get_output_fps(&self) -> Result<u32, RenderError> {
    if (&self.output_fps).is_some() {
      return Ok(self.output_fps.unwrap());
    }
    if (&self.clips).len() == 0 {
      return Err(RenderError::NoClips)
    }

    (&self.clips)
      .iter()
      .map(|clip| clip.get_fps())
      .map(|fps_result| fps_result.map_err(|e| RenderError::Video(e)))
      .collect::<Result<Vec<u32>, _>>()?
      .into_iter()
      .max()
      .ok_or(RenderError::NoClips)
  }

  pub fn write_video(&self) {
    // get ffmpeg to render the video
    let mut ffmpeg = Command::new("ffmpeg");
    // ffmpeg -f lavfi -i color=c=red:s=1920x1080:r=30 -t 10 red_video.mp4
    // ffmpeg.arg("-y")
    //   .arg("-f")
    //   .arg("lavfi")
    //   .arg("-i")
    //   .arg("color=c=red:s=1920x1080:r=30")
    //   .arg("-t")
    //   .arg("30")
    //   .arg(&self.output_path);

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

    // ffmpeg.wait().unwrap();

    if let Some(stdin) = ffmpeg.stdin.as_mut() {
      // let desired_clip = self.clips.get(0).unwrap();

      let frame_size = 1920 * 1080 * 3; // width * height * channels (RGB)
      let mut frame = vec![0u8; frame_size];

      for i in 0..30 * 30 {
        let r = 255;
        let g = 0;
        let b = 0;

        for pixel in frame.chunks_exact_mut(3) {
          pixel[0] = r;
          pixel[1] = g;
          pixel[2] = b;
        }

        stdin.write_all(&frame).expect("Failed to write to stdin");
      }

      stdin.flush().expect("Failed to flush stdin");

      ffmpeg.wait_with_output().unwrap();
    }
  }
}
