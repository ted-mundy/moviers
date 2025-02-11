use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipError {
    #[error("Invalid data")]
    InvalidData,
}

pub trait VideoClip {
    /// Get the frame at the given frame number.
    /// This returning vector should be the raw pixel data of the frame.
    ///
    /// It should be:
    /// [r, g, b, r, g, b, r, g, b, ...], from the top-left corner of the frame to the bottom-right.
    /// of size w*h*3
    /// TODO: This should eventually be w*h*4, but we don't support alpha yet.
    fn get_frame(&self, frame_number: usize) -> Result<Vec<u8>, ClipError>;

    /// Gets if we can skip doing the frame-by-frame rendering and just use ffmpeg to render the video. Very useful
    /// for performance reasons in instances such as simple things like a single solid colour on a video, trimming
    /// a video, changing the resolution/output format, etc.
    ///
    /// TODO: This should actually get used at some point.
    fn get_can_use_ffmpeg(&self) -> bool {
        true
    }

    fn get_duration(&self) -> f64;

    fn get_fps(&self) -> Result<u32, ClipError>;

    fn get_size(&self) -> [u32; 2];

    fn get_start_time(&self) -> f64;

    fn get_position(&self) -> [i32; 2];
}
