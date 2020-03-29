use crate::time::Time;

/// A subtitle item
#[derive(Debug)]
pub struct Subtitle {
    /// A number indicating which subtitle it is in the sequence
    pub pos: usize,
    /// The time that the subtitle should appear
    pub start_time: Time,
    /// The time that the subtitle should disappear
    pub end_time: Time,
    /// The subtitle itself
    pub text: String,
}
