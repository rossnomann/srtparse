use crate::{
    item::{Item, ItemFactory, ItemFactoryError},
    time::ParseTimeError,
};
use std::{
    error::Error,
    fmt,
    io::{BufRead, Error as IoError, Lines},
    num::ParseIntError,
};

const UTF8_BOM: &str = "\u{feff}";
const TIME_DELIMITER: &str = "-->";

/// Subtitles parser
pub struct Parser<B> {
    lines: Lines<B>,
    state: State,
    factory: ItemFactory,
}

impl<B> Parser<B>
where
    B: BufRead,
{
    /// Creates a new parser from a buffered reader
    pub fn new(reader: B) -> Self {
        Parser {
            lines: reader.lines(),
            state: State::Start,
            factory: ItemFactory::default(),
        }
    }

    fn read_line(&mut self) -> Result<Option<String>, ParseError> {
        self.lines.next().transpose().map_err(ParseError::ReadLine)
    }

    fn parse_item(&mut self) -> Result<Option<Item>, ParseError> {
        use self::State::*;
        loop {
            match &self.state {
                Start => {
                    let line = match self.read_line()? {
                        Some(line) => line,
                        None => {
                            return Ok(None);
                        }
                    };
                    self.state = Pos(String::from(line.trim_start_matches(UTF8_BOM).trim()));
                }
                Pos(line) => {
                    if self.factory.maybe_ready() {
                        return Ok(Some(self.factory.take()?));
                    }
                    let pos = line.parse::<usize>().map_err(ParseError::BadPosition)?;
                    self.factory.set_pos(pos);
                    self.state = Time;
                }
                Time => {
                    let line = match self.read_line()? {
                        Some(line) => line,
                        None => return Err(ParseError::UnexpectedEnd),
                    };
                    let mut parts = line.trim().split(TIME_DELIMITER);
                    if let Some(v) = parts.next() {
                        self.factory
                            .set_start_time(v.parse().map_err(ParseError::ParseTimeStart)?);
                    }
                    if let Some(v) = parts.next() {
                        self.factory.set_end_time(v.parse().map_err(ParseError::ParseTimeEnd)?);
                    }
                    if let Some(part) = parts.next() {
                        return Err(ParseError::ExtraTimePart(String::from(part)));
                    }
                    self.state = Text;
                }
                Text => match self.read_line()? {
                    Some(line) => {
                        let line = line.trim();
                        if line.is_empty() {
                            match self.read_line()? {
                                Some(line) => {
                                    self.state = Pos(String::from(line.trim()));
                                }
                                None => {
                                    self.state = Stop;
                                    return Ok(Some(self.factory.take()?));
                                }
                            }
                        } else {
                            self.factory.append_text(line);
                        }
                    }
                    None => {
                        self.state = Stop;
                        return Ok(Some(self.factory.take()?));
                    }
                },
                Stop => return Ok(None),
            }
        }
    }
}

#[derive(Clone, Debug)]
enum State {
    Start,
    Pos(String),
    Time,
    Text,
    Stop,
}

impl<B> Iterator for Parser<B>
where
    B: BufRead,
{
    type Item = Result<Item, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_item().transpose()
    }
}

/// An error when parsing a subtitle
#[derive(Debug)]
pub enum ParseError {
    /// An error when parsing subtitle position
    BadPosition(ParseIntError),
    /// Can not create subtitle item
    CreateSubtitle(ItemFactoryError),
    /// An extra time part found in subtitle, there should be start and end only
    ExtraTimePart(String),
    /// Could not parse start time
    ParseTimeStart(ParseTimeError),
    /// Could not parse end time
    ParseTimeEnd(ParseTimeError),
    /// Could not read a line
    ReadLine(IoError),
    /// Input ends unexpectedly
    UnexpectedEnd,
}

impl From<ItemFactoryError> for ParseError {
    fn from(err: ItemFactoryError) -> Self {
        ParseError::CreateSubtitle(err)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match self {
            BadPosition(err) => write!(out, "bad subtitle position: {err}"),
            CreateSubtitle(err) => write!(out, "{err}"),
            ExtraTimePart(part) => write!(
                out,
                "an extra time part found: '{part}'; there should be start and end only"
            ),
            ParseTimeStart(err) => write!(out, "failed to parse start time: {err}"),
            ParseTimeEnd(err) => write!(out, "failed to parse end time: {err}"),
            ReadLine(err) => write!(out, "could not read a line from input: {err}"),
            UnexpectedEnd => write!(out, "unexpected end of input"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::ParseError::*;
        Some(match self {
            BadPosition(err) => err,
            CreateSubtitle(err) => err,
            ExtraTimePart(_part) => return None,
            ParseTimeStart(err) => err,
            ParseTimeEnd(err) => err,
            ReadLine(err) => err,
            UnexpectedEnd => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::Time;
    use std::io::Cursor;

    fn parse_ok(data: &str) -> Vec<Item> {
        let parser = Parser::new(Cursor::new(data));
        parser.map(|x| x.unwrap()).collect()
    }

    fn parse_err(data: &str) -> String {
        let mut parser = Parser::new(Cursor::new(data));
        parser.next().unwrap().unwrap_err().to_string()
    }

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
        let source_with_bom = format!("{UTF8_BOM}{source_without_bom}");

        fn assert_it_works(data: &str) {
            let result = parse_ok(data);
            assert_eq!(result.len(), 4);
            assert_eq!(
                result[0],
                Item {
                    pos: 1,
                    start_time: Time {
                        hours: 0,
                        minutes: 0,
                        seconds: 58,
                        milliseconds: 392
                    },
                    end_time: Time {
                        hours: 0,
                        minutes: 1,
                        seconds: 2,
                        milliseconds: 563
                    },
                    text: String::from("The war had all but ground to a halt\nin the blink of an eye.")
                }
            );

            assert_eq!(
                result[1],
                Item {
                    pos: 2,
                    start_time: Time {
                        hours: 0,
                        minutes: 1,
                        seconds: 4,
                        milliseconds: 565
                    },
                    end_time: Time {
                        hours: 0,
                        minutes: 1,
                        seconds: 8,
                        milliseconds: 986
                    },
                    text: String::from("Lucian, the most feared and ruthless\nleader ever to rule the Lycan clan...")
                }
            );

            assert_eq!(
                result[2],
                Item {
                    pos: 3,
                    start_time: Time {
                        hours: 0,
                        minutes: 1,
                        seconds: 9,
                        milliseconds: 70
                    },
                    end_time: Time {
                        hours: 0,
                        minutes: 1,
                        seconds: 11,
                        milliseconds: 656
                    },
                    text: String::from("...had finally been killed.")
                }
            );

            assert_eq!(
                result[3],
                Item {
                    pos: 652,
                    start_time: Time {
                        hours: 1,
                        minutes: 53,
                        seconds: 2,
                        milliseconds: 325
                    },
                    end_time: Time {
                        hours: 1,
                        minutes: 53,
                        seconds: 6,
                        milliseconds: 162
                    },
                    text: String::from("Soon, Marcus will take the throne.")
                }
            );
        }

        assert_it_works(source_without_bom);
        assert_it_works(&source_with_bom);
        assert_eq!(parse_ok("").len(), 0);
    }

    #[test]
    fn it_fails_with_bad_position() {
        let err = parse_err("bad position");
        assert_eq!(err, "bad subtitle position: invalid digit found in string");
    }

    #[test]
    fn it_fails_with_bad_start_time() {
        let err = parse_err("1\nbad time");
        assert_eq!(
            err,
            "failed to parse start time: could not parse hours: invalid digit found in string"
        );
    }

    #[test]
    fn it_fails_with_bad_end_time() {
        let err = parse_err("1\n00:00:58,392 --> bad end time");
        assert_eq!(
            err,
            "failed to parse end time: could not parse hours: invalid digit found in string"
        );
    }

    #[test]
    fn it_fails_with_bad_time_format() {
        let err = parse_err("1\n00:00:00:00");
        assert_eq!(err, "failed to parse start time: unexpected time part: \'00\'");
    }

    #[test]
    fn it_fails_with_extra_time() {
        let err = parse_err("1\n00:00:58,392 --> 00:01:02,563 --> 00:01:02,563");
        assert_eq!(
            err,
            "an extra time part found: \' 00:01:02,563\'; there should be start and end only"
        );
    }

    #[test]
    fn it_fails_with_missing_start_time() {
        let err = parse_err("1");
        assert_eq!(err, "unexpected end of input");
    }

    #[test]
    fn it_fails_with_missing_end_time() {
        let err = parse_err("1\n00:00:58,392");
        assert_eq!(err, "item end time is missing");
    }

    #[test]
    fn it_fails_with_missing_text() {
        let err = parse_err("1\n00:00:58,392 --> 00:01:02,563");
        assert_eq!(err, "item text is missing");
    }
}
