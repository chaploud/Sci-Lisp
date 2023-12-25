/* core/types/error.rs */

use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;
use std::fmt::{self, Debug};

use crate::core::parse::Rule;

// use everywhere
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // wrapped errors
    ParseBool(ParseBoolError),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    PestParse(pest::error::Error<Rule>),
    Regex(regex::Error),
    IO(std::io::Error),
    Readline(rustyline::error::ReadlineError),

    // custom errors
    Name(String),
    Type(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            ParseBool(err) => write!(f, "Parse Bool Error: {:#?}", err),
            ParseInt(err) => write!(f, "Parse Int Error: {:#?}", err),
            ParseFloat(err) => write!(f, "Parse Float Error: {:#?}", err),
            PestParse(err) => write!(f, "Pest Parse Error: {:#?}", err),
            Regex(err) => write!(f, "Regex Error: {:#?}", err),
            IO(err) => write!(f, "IO Error: {:#?}", err),
            Readline(err) => write!(f, "Readline Error: {:#?}", err),
            Name(msg) => write!(f, "Name Error: '{:#?}' is not defined", msg),
            Type(expected, actual) => {
                write!(f, "Type Error: expected {:#?}, got {:#?}", expected, actual)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;
        match *self {
            ParseBool(ref err) => Some(err),
            ParseInt(ref err) => Some(err),
            ParseFloat(ref err) => Some(err),
            PestParse(ref err) => Some(err),
            Regex(ref err) => Some(err),
            IO(ref err) => Some(err),
            Readline(ref err) => Some(err),
            Type(_, _) => None,
            Name(_) => None,
        }
    }
}


// impl for custom errors

impl From<ParseBoolError> for Error {
    fn from(err: ParseBoolError) -> Self {
        Error::ParseBool(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
        Error::ParseFloat(err)
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Self {
        Error::PestParse(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<rustyline::error::ReadlineError> for Error {
    fn from(err: rustyline::error::ReadlineError) -> Self {
        Error::Readline(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Regex(err)
    }
}
