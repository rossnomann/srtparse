use std::io::{Error as IoError, Read};
use std::fs::File;
use std::num::ParseIntError;
use std::path::Path;
use std::time::Duration;

// TODO: ADD DOCS!!!

const UTF8_BOM: &'static str = "\u{feff}";

#[derive(Debug)]
enum State {
    Pos,
    Time,
    Text,
}

#[derive(Debug)]
pub struct Subtitle {
    pub pos: usize,
    pub start_time: Duration,
    pub end_time: Duration,
    pub text: String,
}

pub fn parse(source: &str) -> Result<Vec<Subtitle>, Error> {
    let source = source.trim_left_matches(UTF8_BOM).trim().lines();

    let mut result = Vec::new();

    let mut state = State::Pos;

    let mut pos: Option<usize> = None;
    let mut start_time: Option<Duration> = None;
    let mut end_time: Option<Duration> = None;
    let mut text: Option<String> = None;

    macro_rules! push_subtitle {
        ($pos:ident) => (
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
                }
            });
        )
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
                pos = Some(try!(line.parse::<usize>()));
                state = State::Time;
            }
            State::Time => {
                let mut parts = line.split("-->");
                start_time = match parts.next() {
                    Some(v) => Some(try!(duration_from_str(v))),
                    None => None,
                };
                end_time = match parts.next() {
                    Some(v) => Some(try!(duration_from_str(v))),
                    None => None,
                };
                state = State::Text;
            }
            State::Text => {
                if line.is_empty() {
                    state = State::Pos;
                } else {
                    if let Some(ref mut txt) = text {
                        txt.push('\n');
                        txt.push_str(line);
                    } else {
                        text = Some(line.to_string());
                    }
                }
            }
        }
    }
    if let Some(ps) = pos {
        push_subtitle!(ps);
    }

    Ok(result)
}

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Subtitle>, Error> {
    let mut file = try!(File::open(path));
    let mut buf = String::new();
    try!(file.read_to_string(&mut buf));
    parse(&buf)
}

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    ParseInt(ParseIntError),
    MissingText,
    MissingEndTime,
    MissingStartTime,
    ParseTime,
}

// TODO: impl StdError

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

macro_rules! parse_time_part {
    ($part:expr) => {{
        match $part {
            Some(val) => {
                try!(val.trim().parse::<u64>())
            },
            None => {
                return Err(Error::ParseTime);
            }
        }
    }}
}

fn duration_from_str(time: &str) -> Result<Duration, Error> {
    let mut time = time.split(",");
    let (hours, mut minutes, mut seconds) = match time.next() {
        Some(val) => {
            let mut parts = val.split(":");
            let result = (
                parse_time_part!(parts.next()),
                parse_time_part!(parts.next()),
                parse_time_part!(parts.next()),
            );
            if parts.next().is_some() {
                return Err(Error::ParseTime);
            }
            result
        },
        None => {
            return Err(Error::ParseTime);
        }
    };
    let mut milliseconds = parse_time_part!(time.next());
    if time.next().is_some() {
        return Err(Error::ParseTime);
    }
    minutes += hours * 60;
    seconds += minutes * 60;
    milliseconds += seconds * 1000;
    Ok(Duration::new(milliseconds, 0))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use ::{Error, UTF8_BOM, parse};

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

        fn assert_it_works(data: &str) -> Result<(), Error> {
            let result = try!(parse(&data));
            assert_eq!(result.len(), 4);
            assert_eq!(result[0].pos, 1);
            assert_eq!(result[0].start_time, Duration::new(58392, 0));
            assert_eq!(result[0].end_time, Duration::new(62563, 0));
            assert_eq!(result[0].text, "The war had all but ground to a halt\nin the blink of an eye.");
            assert_eq!(result[1].pos, 2);
            assert_eq!(result[1].start_time, Duration::new(64565, 0));
            assert_eq!(result[1].end_time, Duration::new(68986, 0));
            assert_eq!(result[1].text, "Lucian, the most feared and ruthless\nleader ever to rule the Lycan clan...");
            assert_eq!(result[2].pos, 3);
            assert_eq!(result[2].start_time, Duration::new(69070, 0));
            assert_eq!(result[2].end_time, Duration::new(71656, 0));
            assert_eq!(result[2].text, "...had finally been killed.");
            assert_eq!(result[3].pos, 652);
            assert_eq!(result[3].start_time, Duration::new(6782325, 0));
            assert_eq!(result[3].end_time, Duration::new(6786162, 0));
            assert_eq!(result[3].text, "Soon, Marcus will take the throne.");
            Ok(())
        }

        assert_it_works(&source_without_bom).expect("Failed to parse UTF-8 source without BOM");
        assert_it_works(&source_with_bom).expect("Failed to parse UTF-8 source with BOM");
    }

    // TODO: NEED MOAR TESTS!!!

}
