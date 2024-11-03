use std::fmt;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum RESPError {
    FromUtf8,
    OutOfBounds(usize),
    WrongType,
}

impl fmt::Display for RESPError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RESPError::OutOfBounds(index) => write!(f, "Out of bounds at index {}", index),
            RESPError::FromUtf8 => write!(f, "Cannot convert to utf8"),
            RESPError::WrongType => write!(f, "Wrong prefix for RESP type"),
        }
    }
}

impl From<FromUtf8Error> for RESPError {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8
    }
}

pub type RESPResult<T> = Result<T, RESPError>;
