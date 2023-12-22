use std::{
    cmp::Ord,
    fmt::{self, Debug},
    hash::Hash,
    marker::Copy,
    string::ToString,
};

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Type(String, String),
    Name(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            Parse(msg) => write!(f, "Parse Error: {}", msg),
            Type(expected, actual) => {
                write!(f, "Type Error: expected {}, got {}", expected, actual)
            }
            Name(msg) => write!(f, "Name Error: '{}' is not defined", msg),
        }
    }
}

impl<T> From<pest::error::Error<T>> for Error
where
    T: Debug + Ord + Copy + Hash,
{
    fn from(err: pest::error::Error<T>) -> Self {
        Error::Parse(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Parse(err.to_string())
    }
}
