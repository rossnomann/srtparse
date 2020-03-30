//! A library for reading [SRT Subtitles][1].
//!
//! # Examples
//!
//! ## Reading from a string
//!
//! ```
//! let subtitles = srtparse::from_str("1\n00:00:01,100 --> 00:00:02,120\nHello!").unwrap();
//! println!("{:?}", subtitles);
//! ```
//!
//! ## Reading from a file
//!
//! ```
//! let subtitles = srtparse::from_file("./data/underworld.srt").unwrap();
//! println!("{:?}", subtitles[0]);
//! ```
//!
//! [1]: https://matroska.org/technical/specs/subtitles/srt.html
#![warn(missing_docs)]

pub use self::{
    parser::ParseError,
    reader::{from_file, from_reader, from_str, ReaderError},
    subtitle::{Subtitle, SubtitleFactoryError},
    time::{ParseTimeError, Time},
};

mod parser;
mod reader;
mod subtitle;
mod time;
