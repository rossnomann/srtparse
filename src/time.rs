use std::{error::Error, fmt, num::ParseIntError, str::FromStr, time::Duration};

/// Describes the time when subtitle should appear or disappear
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
    /// Number of hours
    pub hours: u64,
    /// Number of minutes
    pub minutes: u64,
    /// Number of seconds
    pub seconds: u64,
    /// Number of milliseconds
    pub milliseconds: u64,
}

impl Time {
    /// Converts `Time` to `Duration` from standard library
    pub fn into_duration(self) -> Duration {
        let minutes = self.minutes + (self.hours * 60);
        let seconds = self.seconds + (minutes * 60);
        let milliseconds = self.milliseconds + (seconds * 1000);
        Duration::from_millis(milliseconds)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(
            out,
            "{:02}:{:02}:{:02},{}",
            self.hours, self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl FromStr for Time {
    type Err = ParseTimeError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut raw = raw.trim().split(',');
        let (hours, minutes, seconds) = match raw.next() {
            Some(raw_time) => {
                let mut raw_time = raw_time.split(':');
                let hours = match raw_time.next() {
                    Some(hours) => hours.parse::<u64>().map_err(ParseTimeError::ParseHours)?,
                    None => return Err(ParseTimeError::MissingHours),
                };
                let minutes = match raw_time.next() {
                    Some(minutes) => minutes.parse::<u64>().map_err(ParseTimeError::ParseMinutes)?,
                    None => return Err(ParseTimeError::MissingMinutes),
                };
                let seconds = match raw_time.next() {
                    Some(seconds) => seconds.parse::<u64>().map_err(ParseTimeError::ParseSeconds)?,
                    None => return Err(ParseTimeError::MissingSeconds),
                };
                if let Some(part) = raw_time.next() {
                    return Err(ParseTimeError::UnexpectedTimePart(String::from(part)));
                }
                (hours, minutes, seconds)
            }
            None => return Err(ParseTimeError::MissingTime),
        };
        let milliseconds = match raw.next() {
            Some(value) => value.parse::<u64>().map_err(ParseTimeError::ParseMilliseconds)?,
            None => return Err(ParseTimeError::MissingMilliseconds),
        };
        if let Some(part) = raw.next() {
            return Err(ParseTimeError::UnexpectedTimePart(String::from(part)));
        }
        Ok(Self {
            hours,
            minutes,
            seconds,
            milliseconds,
        })
    }
}

/// An error when parsing time
#[derive(Debug)]
pub enum ParseTimeError {
    /// Hours does not contain an integer
    ParseHours(ParseIntError),
    /// Milliseconds does not contain an integer
    ParseMilliseconds(ParseIntError),
    /// Minutes does not contain an integer
    ParseMinutes(ParseIntError),
    /// Seconds does not contain an integer
    ParseSeconds(ParseIntError),
    /// Hours not found in time part
    MissingHours,
    /// Milliseconds not found in time part
    MissingMilliseconds,
    /// Minutes not found in time part
    MissingMinutes,
    /// Seconds not found in time part
    MissingSeconds,
    /// Time part is empty
    MissingTime,
    /// Got an unexpected part of time
    UnexpectedTimePart(String),
}

impl fmt::Display for ParseTimeError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseTimeError::*;
        match self {
            ParseHours(err) => write!(out, "could not parse hours: {err}"),
            ParseMinutes(err) => write!(out, "could not parse minutes: {err}"),
            ParseSeconds(err) => write!(out, "could not parse seconds: {err}"),
            ParseMilliseconds(err) => write!(out, "could not parse milliseconds: {err}"),
            MissingHours => write!(out, "hours not found"),
            MissingMinutes => write!(out, "minutes not found"),
            MissingSeconds => write!(out, "seconds not found"),
            MissingMilliseconds => write!(out, "milliseconds not found"),
            MissingTime => write!(out, "time not found"),
            UnexpectedTimePart(part) => write!(out, "unexpected time part: '{part}'"),
        }
    }
}

impl Error for ParseTimeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::ParseTimeError::*;
        if let ParseHours(err) | ParseMinutes(err) | ParseSeconds(err) | ParseMilliseconds(err) = self {
            Some(err)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            "".parse::<Time>().unwrap_err().to_string(),
            "could not parse hours: cannot parse integer from empty string"
        );
        assert_eq!(
            "x".parse::<Time>().unwrap_err().to_string(),
            "could not parse hours: invalid digit found in string"
        );
        assert_eq!(
            "x,x".parse::<Time>().unwrap_err().to_string(),
            "could not parse hours: invalid digit found in string"
        );
        assert_eq!("1,x".parse::<Time>().unwrap_err().to_string(), "minutes not found");
        assert_eq!(
            "00:01:02,200".parse::<Time>().unwrap(),
            Time {
                hours: 0,
                minutes: 1,
                seconds: 2,
                milliseconds: 200
            }
        );
    }

    #[test]
    fn display() {
        let time = Time {
            hours: 0,
            minutes: 1,
            seconds: 2,
            milliseconds: 200,
        };
        assert_eq!(time.to_string(), "00:01:02,200");
    }

    #[test]
    fn into_duration() {
        let time = Time {
            hours: 0,
            minutes: 1,
            seconds: 2,
            milliseconds: 200,
        };
        assert_eq!(time.into_duration(), Duration::from_millis(62200));
    }
}
