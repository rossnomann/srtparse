use crate::{
    item::Item,
    parser::{ParseError, Parser},
};
use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader, Cursor, Error as IoError},
    path::Path,
};

/// Read subtitles from a string
pub fn from_str<S>(input: S) -> Result<Vec<Item>, ReaderError>
where
    S: AsRef<[u8]>,
{
    from_reader(Cursor::new(input))
}

/// Read subtitles from a file
pub fn from_file(path: impl AsRef<Path>) -> Result<Vec<Item>, ReaderError> {
    from_reader(BufReader::new(File::open(path).map_err(ReaderError::OpenFile)?))
}

/// Read subtitles from a buffered reader
pub fn from_reader(reader: impl BufRead) -> Result<Vec<Item>, ReaderError> {
    let parser = Parser::new(reader);
    let mut result = Vec::new();
    for item in parser {
        let item = item?;
        result.push(item);
    }
    Ok(result)
}

/// An error when reading subtitles
#[derive(Debug)]
pub enum ReaderError {
    /// Could not open a file
    OpenFile(IoError),
    /// Failed to parse subtitles
    Parse(ParseError),
}

impl From<ParseError> for ReaderError {
    fn from(err: ParseError) -> Self {
        ReaderError::Parse(err)
    }
}

impl fmt::Display for ReaderError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ReaderError::*;
        match self {
            OpenFile(err) => write!(out, "could not open a file: {err}"),
            Parse(err) => write!(out, "parse error: {err}"),
        }
    }
}

impl Error for ReaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::ReaderError::*;
        match self {
            OpenFile(err) => Some(err),
            Parse(err) => Some(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn read_from_file_failed() {
        let err = from_file("/file/does/not/exist").unwrap_err().to_string();
        assert_eq!(err, "could not open a file: No such file or directory (os error 2)");
    }

    #[test]
    fn read_from_file_success() {
        let result = from_file("./data/underworld.srt").unwrap();
        assert_eq!(result.len(), 706);

        let first = result.first().unwrap();
        assert_eq!(first.pos, 1);
        assert_eq!(first.start_time.into_duration(), Duration::from_millis(58392));
        assert_eq!(first.end_time.into_duration(), Duration::from_millis(61478));
        assert_eq!(first.text, "Война закончилась в мгновение ока.");

        let last = result.last().unwrap();
        assert_eq!(last.pos, 706);
        assert_eq!(last.start_time.into_duration(), Duration::from_millis(6_801_628));
        assert_eq!(last.end_time.into_duration(), Duration::from_millis(6_804_381));
        assert_eq!(last.text, "... будет объявлена охота.");
    }
}
