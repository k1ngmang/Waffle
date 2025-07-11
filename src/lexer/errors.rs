use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum LexError {
    Io(std::io::Error),
    Utf8(std::str::Utf8Error),
    UnexpectedEof,
    IndexOutOfRange,
    FailedToParseNumber,
}

impl From<std::io::Error> for LexError {
    fn from(e: std::io::Error) -> Self {
        LexError::Io(e)
    }
}

impl From<std::str::Utf8Error> for LexError {
    fn from(e: std::str::Utf8Error) -> Self {
        LexError::Utf8(e)
    }
}

impl From<ParseFloatError> for LexError {
    fn from(e: ParseFloatError) -> Self {
        LexError::FailedToParseNumber
    }
}
impl From<ParseIntError> for LexError {
    fn from(e: ParseIntError) -> Self {
        LexError::FailedToParseNumber
    }
}
