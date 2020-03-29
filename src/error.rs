use std::{error::Error as StdError, fmt, io::Error as IoError};

/// Describes all errors that may occur
#[derive(Debug)]
pub enum Error {
    /// An error when parsing subtitle position
    BadPosition,
    /// An error when parsing subtitle time
    BadTime,
    /// Unsupported subtitle time format
    BadTimeFormat,
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
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::OpenFile(ref err) => Some(err),
            Error::ReadFile(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadPosition => write!(out, "Invalid subtitle position"),
            Error::BadTime => write!(out, "Invalid subtitle time"),
            Error::BadTimeFormat => write!(out, "Invalid subtitle time format"),
            Error::MissingStartTime => write!(out, "Subtitle start time is missing"),
            Error::MissingEndTime => write!(out, "Subtitle end time is missing"),
            Error::MissingText => write!(out, "Subtitle text is missing"),
            Error::OpenFile(ref err) => write!(out, "{}", err),
            Error::ReadFile(ref err) => write!(out, "{}", err),
        }
    }
}
