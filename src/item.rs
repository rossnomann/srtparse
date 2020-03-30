use crate::time::Time;
use std::{error::Error, fmt};

/// A subtitle item
#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    /// A number indicating which subtitle it is in the sequence
    pub pos: usize,
    /// The time that the subtitle should appear
    pub start_time: Time,
    /// The time that the subtitle should disappear
    pub end_time: Time,
    /// The subtitle itself
    pub text: String,
}

impl fmt::Display for Item {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(
            out,
            "{}\n{}-->{}\n{}",
            self.pos, self.start_time, self.end_time, self.text
        )
    }
}

#[derive(Default)]
pub(super) struct ItemFactory {
    pos: Option<usize>,
    start_time: Option<Time>,
    end_time: Option<Time>,
    text: Option<String>,
}

impl ItemFactory {
    pub(super) fn set_pos(&mut self, pos: usize) {
        self.pos = Some(pos);
    }

    pub(super) fn set_start_time(&mut self, start_time: Time) {
        self.start_time = Some(start_time);
    }

    pub(super) fn set_end_time(&mut self, end_time: Time) {
        self.end_time = Some(end_time);
    }

    pub(super) fn append_text<P: AsRef<str>>(&mut self, part: P) {
        let part = part.as_ref();
        match self.text.as_mut() {
            Some(text) => {
                text.push('\n');
                text.push_str(part);
            }
            None => {
                self.text = Some(String::from(part));
            }
        }
    }

    pub(super) fn maybe_ready(&self) -> bool {
        self.pos.is_some()
    }

    pub(super) fn take(&mut self) -> Result<Item, ItemFactoryError> {
        Ok(Item {
            pos: self.pos.take().ok_or(ItemFactoryError::NoPosition)?,
            start_time: self.start_time.take().ok_or(ItemFactoryError::NoStartTime)?,
            end_time: self.end_time.take().ok_or(ItemFactoryError::NoEndTime)?,
            text: self.text.take().ok_or(ItemFactoryError::NoText)?,
        })
    }
}

/// Could not create subtitle
#[derive(Debug)]
pub enum ItemFactoryError {
    /// Subtitle position is missing
    NoPosition,
    /// Subtitle start time is missing
    NoStartTime,
    /// Subtitle end time is missing
    NoEndTime,
    /// Subtitle text is missing
    NoText,
}

impl fmt::Display for ItemFactoryError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ItemFactoryError::*;
        match self {
            NoPosition => write!(out, "item position is missing"),
            NoStartTime => write!(out, "item start time is missing"),
            NoEndTime => write!(out, "item end time is missing"),
            NoText => write!(out, "item text is missing"),
        }
    }
}

impl Error for ItemFactoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let item = Item {
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
        assert_eq!(item.to_string(), "1\n00:00:05,200-->00:00:06,300\ntest");
    }
}
