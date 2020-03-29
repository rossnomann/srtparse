use crate::time::Time;
use std::fmt;

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

impl fmt::Display for Subtitle {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(
            out,
            "{}\n{}-->{}\n{}",
            self.pos, self.start_time, self.end_time, self.text
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let subtitle = Subtitle {
            pos: 1,
            start_time: Time {
                hours: 0,
                minutes: 0,
                seconds: 5,
                milliseconds: 200,
            },
            end_time: Time {
                hours: 0,
                minutes: 0,
                seconds: 6,
                milliseconds: 300,
            },
            text: String::from("test"),
        };
        assert_eq!(subtitle.to_string(), "1\n00:00:05,200-->00:00:06,300\ntest");
    }
}
