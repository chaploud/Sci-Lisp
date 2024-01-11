use core::fmt;

use crate::core::value::Value;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Slice {
    pub start: Value,
    pub end: Value,
    pub step: Value,
}

impl Slice {
    pub fn new(start: Value, end: Value, step: Value) -> Self {
        Slice { start, end, step }
    }
}

impl fmt::Display for Slice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let start = if self.start == Value::Nil {
            String::new()
        } else {
            format!("{}", self.start)
        };
        let end = if self.end == Value::Nil {
            String::new()
        } else {
            format!("{}", self.end)
        };
        let step = if self.step == Value::Nil {
            String::new()
        } else {
            format!("{}", self.step)
        };
        write!(f, "{}|{}|{}", start, end, step)
    }
}
