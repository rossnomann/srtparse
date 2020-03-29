//! A library for reading [SRT Subtitles][1].
//!
//! # Examples
//!
//! ## Reading from string
//!
//! ```
//! use srtparse::parse;
//! let subtitles = parse("1\n00:00:01,100 --> 00:00:02,120\nHello!").unwrap();
//! println!("{:?}", subtitles);
//! ```
//!
//! ## Reading from file
//!
//! ```
//! use srtparse::read_from_file;
//! let subtitles = read_from_file("./data/underworld.srt").unwrap();
//! println!("{:?}", subtitles[0]);
//! ```
//!
//! [1]: https://matroska.org/technical/specs/subtitles/srt.html
#![warn(missing_docs)]
use std::{
    error::Error as StdError,
    fmt,
    fs::File,
    io::{Error as IoError, Read},
    path::Path,
    result::Result as StdResult,
    time::Duration,
};

const UTF8_BOM: &str = "\u{feff}";

#[derive(Debug)]
enum State {
    Pos,
    Time,
    Text,
}

/// A subtitle item
#[derive(Debug)]
pub struct Subtitle {
    /// A number indicating which subtitle it is in the sequence
    pub pos: usize,
    /// The time that the subtitle should appear
    pub start_time: Duration,
    /// The time that the subtitle should disappear
    pub end_time: Duration,
    /// The subtitle itself
    pub text: String,
}

/// Read subtitles from a string
pub fn parse(source: &str) -> Result<Vec<Subtitle>> {
    let source = source.trim_start_matches(UTF8_BOM).trim().lines();

    let mut result = Vec::new();

    let mut state = State::Pos;

    let mut pos: Option<usize> = None;
    let mut start_time: Option<Duration> = None;
    let mut end_time: Option<Duration> = None;
    let mut text: Option<String> = None;

    macro_rules! push_subtitle {
        ($pos:ident) => {
            result.push(Subtitle {
                pos: $pos,
                start_time: match start_time {
                    Some(val) => val,
                    None => return Err(Error::MissingStartTime),
                },
                end_time: match end_time {
                    Some(val) => val,
                    None => return Err(Error::MissingEndTime),
                },
                text: match text {
                    Some(val) => val,
                    None => return Err(Error::MissingText),
                },
            });
        };
    }

    for line in source {
        match state {
            State::Pos => {
                if let Some(ps) = pos {
                    push_subtitle!(ps);
                    start_time = None;
                    end_time = None;
                    text = None;
                }
                pos = Some(line.parse::<usize>().map_err(|_| Error::BadPosition)?);
                state = State::Time;
            }
            State::Time => {
                let mut parts = line.split("-->");
                start_time = match parts.next() {
                    Some(v) => Some(duration_from_str(v)?),
                    None => None,
                };
                end_time = match parts.next() {
                    Some(v) => Some(duration_from_str(v)?),
                    None => None,
                };
                if parts.next().is_some() {
                    return Err(Error::BadTime);
                }
                state = State::Text;
            }
            State::Text => {
                if line.is_empty() {
                    state = State::Pos;
                } else if let Some(ref mut txt) = text {
                    txt.push('\n');
                    txt.push_str(line);
                } else {
                    text = Some(line.to_string());
                }
            }
        }
    }
    if let Some(ps) = pos {
        push_subtitle!(ps);
    }

    Ok(result)
}

/// Read subtitles from a file
pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Subtitle>> {
    let mut file = File::open(path).map_err(Error::OpenFile)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).map_err(Error::ReadFile)?;
    parse(&buf)
}

/// Alias for std result
pub type Result<T> = StdResult<T, Error>;

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

macro_rules! parse_time_part {
    ($part:expr) => {{
        match $part {
            Some(val) => val.trim().parse::<u64>().map_err(|_| Error::BadTime)?,
            None => {
                return Err(Error::BadTimeFormat);
            }
        }
    }};
}

fn duration_from_str(time: &str) -> Result<Duration> {
    let mut time = time.split(',');
    let (hours, mut minutes, mut seconds) = match time.next() {
        Some(val) => {
            let mut parts = val.split(':');
            let result = (
                parse_time_part!(parts.next()),
                parse_time_part!(parts.next()),
                parse_time_part!(parts.next()),
            );
            if parts.next().is_some() {
                return Err(Error::BadTimeFormat);
            }
            result
        }
        None => {
            return Err(Error::BadTimeFormat);
        }
    };
    let mut milliseconds = parse_time_part!(time.next());
    if time.next().is_some() {
        return Err(Error::BadTimeFormat);
    }
    minutes += hours * 60;
    seconds += minutes * 60;
    milliseconds += seconds * 1000;
    Ok(Duration::from_millis(milliseconds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let source_without_bom = "1
00:00:58,392 --> 00:01:02,563
The war had all but ground to a halt
in the blink of an eye.

2
00:01:04,565 --> 00:01:08,986
Lucian, the most feared and ruthless
leader ever to rule the Lycan clan...

3
00:01:09,070 --> 00:01:11,656
...had finally been killed.

652
01:53:02,325 --> 01:53:06,162
Soon, Marcus will take the throne.
";
        let source_with_bom = format!("{}{}", UTF8_BOM, source_without_bom);

        fn assert_it_works(data: &str) -> Result<()> {
            let result = parse(&data)?;
            assert_eq!(result.len(), 4);
            assert_eq!(result[0].pos, 1);
            assert_eq!(result[0].start_time, Duration::from_millis(58392));
            assert_eq!(result[0].end_time, Duration::from_millis(62563));
            assert_eq!(
                result[0].text,
                "The war had all but ground to a halt\nin the blink of an eye."
            );
            assert_eq!(result[1].pos, 2);
            assert_eq!(result[1].start_time, Duration::from_millis(64565));
            assert_eq!(result[1].end_time, Duration::from_millis(68986));
            assert_eq!(
                result[1].text,
                "Lucian, the most feared and ruthless\nleader ever to rule the Lycan clan..."
            );
            assert_eq!(result[2].pos, 3);
            assert_eq!(result[2].start_time, Duration::from_millis(69070));
            assert_eq!(result[2].end_time, Duration::from_millis(71656));
            assert_eq!(result[2].text, "...had finally been killed.");
            assert_eq!(result[3].pos, 652);
            assert_eq!(result[3].start_time, Duration::from_millis(6_782_325));
            assert_eq!(result[3].end_time, Duration::from_millis(6_786_162));
            assert_eq!(result[3].text, "Soon, Marcus will take the throne.");
            Ok(())
        }

        assert_it_works(&source_without_bom).expect("Failed to parse UTF-8 source without BOM");
        assert_it_works(&source_with_bom).expect("Failed to parse UTF-8 source with BOM");
        let empty = parse("").unwrap();
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn it_fails_with_bad_position() {
        let err = parse("bad position").unwrap_err().to_string();
        assert_eq!(err, "Invalid subtitle position");
    }

    #[test]
    fn it_fails_with_bad_start_time() {
        let err = parse("1\nbad time").unwrap_err().to_string();
        assert_eq!(err, "Invalid subtitle time");
    }

    #[test]
    fn it_fails_with_bad_end_time() {
        let err = parse("1\n00:00:58,392 --> bad end time").unwrap_err().to_string();
        assert_eq!(err, "Invalid subtitle time");
    }

    #[test]
    fn it_fails_with_bad_time_format() {
        let err = parse("1\n00:00:00:00").unwrap_err().to_string();
        assert_eq!(err, "Invalid subtitle time format");
    }

    #[test]
    fn it_fails_with_extra_time() {
        let err = parse("1\n00:00:58,392 --> 00:01:02,563 -> 00:01:02,563")
            .unwrap_err()
            .to_string();
        assert_eq!(err, "Invalid subtitle time");
    }

    #[test]
    fn it_fails_with_missing_start_time() {
        let err = parse("1").unwrap_err().to_string();
        assert_eq!(err, "Subtitle start time is missing");
    }

    #[test]
    fn it_fails_with_missing_end_time() {
        let err = parse("1\n00:00:58,392").unwrap_err().to_string();
        assert_eq!(err, "Subtitle end time is missing");
    }

    #[test]
    fn it_fails_with_missing_text() {
        let err = parse("1\n00:00:58,392 --> 00:01:02,563").unwrap_err().to_string();
        assert_eq!(err, "Subtitle text is missing");
    }

    #[test]
    fn read_from_file_failed() {
        let err = read_from_file("/file/does/not/exist").unwrap_err().to_string();
        assert_eq!(err, "No such file or directory (os error 2)");
    }

    #[test]
    fn read_from_file_success() {
        let result = read_from_file("./data/underworld.srt").unwrap();
        assert_eq!(result.len(), 706);

        let first = result.first().unwrap();
        assert_eq!(first.pos, 1);
        assert_eq!(first.start_time, Duration::from_millis(58392));
        assert_eq!(first.end_time, Duration::from_millis(61478));
        assert_eq!(first.text, "Война закончилась в мгновение ока.");

        let last = result.last().unwrap();
        assert_eq!(last.pos, 706);
        assert_eq!(last.start_time, Duration::from_millis(6_801_628));
        assert_eq!(last.end_time, Duration::from_millis(6_804_381));
        assert_eq!(last.text, "... будет объявлена охота.");
    }
}
