use super::clip::Clip;

/// A color clip is a clip that is just a solid color when rendered, with no audio data.
pub struct ColorClip {
  pub color: String,
  pub clip: Clip,
}
