use crate::time::ParseTimeError;
use std::{error::Error as StdError, fmt, io::Error as IoError};

/// Describes all errors that may occur
#[derive(Debug)]
pub enum Error {
    /// An error when parsing subtitle position
    BadPosition,
    /// An extra time part found in subtitle, there should be start and end only
    ExtraTime,
    /// Subtitle start time is missing
    MissingStartTime,
    /// Subtitle end time is missing
    MissingEndTime,
    /// Subtitle text is missing
    MissingText,
    /// Unable to open a file
    OpenFile(IoError),
    /// Unable to read data from a file
    ReadFile(IoError),
    /// Could not parse start time
    ParseTimeStart(ParseTimeError),
    /// Could not parse end time
    ParseTimeEnd(ParseTimeError),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use self::Error::*;
        Some(match self {
            OpenFile(err) => err,
            ReadFile(err) => err,
            ParseTimeStart(err) => err,
            ParseTimeEnd(err) => err,
            _ => return None,
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        match self {
            BadPosition => write!(out, "invalid subtitle position"),
            ExtraTime => write!(out, "an extra time part found, there should be start and end only"),
            MissingStartTime => write!(out, "subtitle start time is missing"),
            MissingEndTime => write!(out, "subtitle end time is missing"),
            MissingText => write!(out, "subtitle text is missing"),
            OpenFile(err) => write!(out, "{}", err),
            ReadFile(err) => write!(out, "{}", err),
            ParseTimeStart(err) => write!(out, "failed to parse start time: {}", err),
            ParseTimeEnd(err) => write!(out, "failed to parse end time: {}", err),
        }
    }
}
