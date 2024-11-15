// use crate::clip::clip;

// #[derive(derive_builder::Builder)]
// pub struct ClipRenderer {
//   clips: Vec<clip::Clip>,
//   pub output_path: String,
// }

// impl ClipRenderer {
//   pub fn push_clip<T>(&mut self, clip: T)
//   where
//     T: clip::ClipTrait,
//   {
//     self.clips.push(clip);
//   }

//   pub fn write_video(&self) {
//     // Write the video to the output path
//     print!("Writing video to {}", self.output_path);
//   }
// }
