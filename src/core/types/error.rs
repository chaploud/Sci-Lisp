/* core/types/error.rs */

use std::fmt::{self, Debug};
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

use crate::core::parse::Rule;
use crate::core::value::Value;

// use everywhere
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // wrapped errors
    ParseBool(ParseBoolError),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    PestParse(Box<pest::error::Error<Rule>>),
    Regex(regex::Error),
    IO(std::io::Error),
    Readline(rustyline::error::ReadlineError),

    // custom errors
    Name(String),
    Type(String),
    Syntax(String),
    Cast(String, String),
    Arity(String),
    Const(String),
    Index(String),
    Key(String),
    Value(String),

    // return
    Return(Value),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            ParseBool(err) => write!(f, "Parse Bool Error: {}", err),
            ParseInt(err) => write!(f, "Parse Int Error: {}", err),
            ParseFloat(err) => write!(f, "Parse Float Error: {}", err),
            PestParse(err) => match err.line_col {
                pest::error::LineColLocation::Pos(pos) => {
                    write!(f, "Parse Error: at {}:{}", pos.0, pos.1)
                }
                pest::error::LineColLocation::Span(start, end) => {
                    write!(f, "Parse Error: at {}:{} to {}:{}", start.0, start.1, end.0, end.1)
                }
            },
            Regex(err) => write!(f, "Regex Error: {}", err),
            IO(err) => write!(f, "IO Error: {}", err),
            Readline(err) => write!(f, "Readline Error: {}", err),
            Name(msg) => write!(f, "Name Error: '{}' is not defined", msg),
            Type(msg) => write!(f, "Type Error: {}", msg),
            Syntax(msg) => write!(f, "Syntax Error: {}", msg),
            Cast(src, dest) => write!(f, "Cast Error: cannot cast {} to {}", src, dest),
            Arity(msg) => write!(f, "Arity Error: {}", msg),
            Const(msg) => write!(f, "Const Error: {}", msg),
            Index(msg) => write!(f, "Index Error: {}", msg),
            Key(msg) => write!(f, "Key Error: {}", msg),
            Value(msg) => write!(f, "Value Error: {}", msg),
            Return(val) => write!(f, "Return: {}", val),
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
            Type(_) => None,
            Name(_) => None,
            Syntax(_) => None,
            Cast(_, _) => None,
            Arity(_) => None,
            Const(_) => None,
            Index(_) => None,
            Key(_) => None,
            Value(_) => None,
            Return(_) => None,
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
        Error::PestParse(Box::new(err))
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

// error helpers
pub fn arity_error(expected: usize, actual: usize) -> Error {
    Error::Arity(format!("expected {} arguments, got {}", expected, actual))
}

pub fn arity_error_range(expected_min: usize, expected_max: usize, actual: usize) -> Error {
    Error::Arity(format!(
        "expected between {} and {} arguments, got {}",
        expected_min, expected_max, actual
    ))
}

pub fn arity_error_min(expected_min: usize, actual: usize) -> Error {
    Error::Arity(format!("expected at least {} arguments, got {}", expected_min, actual))
}

pub fn type_error(expected: &str, actual: &str) -> Error {
    Error::Type(format!("expected type: '{}', got: '{}'", expected, actual))
}

pub fn cannot_compare_error(left: &Value, right: &Value) -> Error {
    Error::Type(format!("cannot compare '{}' and '{}'", left.type_name(), right.type_name(),))
}

pub fn index_out_of_range_error(index: i64) -> Error {
    Error::Index(format!("index '{}' out of range", index))
}

pub fn key_not_found_error(key: Value) -> Error {
    Error::Key(format!("key '{}' not found", key))
}
