use derive_builder::Builder;

#[derive(Builder)]
pub struct Clip {
    pub width: u16,
    pub height: u16,

    /// The URL of the clip. Can be None if the clip does not read from a file, such as a clip of a solid color.
    pub url: Option<String>,
}
