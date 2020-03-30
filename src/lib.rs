//! A library for reading [SRT Subtitles][1].
//!
//! # Examples
//!
//! ## Reading from a string
//!
//! ```
//! let items = srtparse::from_str("1\n00:00:01,100 --> 00:00:02,120\nHello!").unwrap();
//! println!("{:?}", items);
//! ```
//!
//! ## Reading from a file
//!
//! ```
//! let items = srtparse::from_file("./data/underworld.srt").unwrap();
//! println!("{:?}", items[0]);
//! ```
//!
//! [1]: https://matroska.org/technical/specs/subtitles/srt.html
#![warn(missing_docs)]

pub use self::{
    item::{Item, ItemFactoryError},
    parser::ParseError,
    reader::{from_file, from_reader, from_str, ReaderError},
    time::{ParseTimeError, Time},
};

mod item;
mod parser;
mod reader;
mod time;
