pub trait VideoClip {
    /// Get the frame at the given frame number.
    /// This returning vector should be the raw pixel data of the frame.
    ///
    /// It should be:
    /// [r, g, b, r, g, b, r, g, b, ...], from the top-left corner of the frame to the bottom-right.
    fn get_frame(&self, frame_number: usize) -> Result<(u8, u8, u8), String>;

    /// Gets if we can skip doing the frame-by-frame rendering and just use ffmpeg to render the video. Very useful
    /// for performance reasons in instances such as simple things like a single solid colour on a video, trimming
    /// a video, changing the resolution/output format, etc.
    ///
    /// TODO: This should actually get used at some point.
    fn get_can_use_ffmpeg(&self) -> bool {
        true
    }
}
