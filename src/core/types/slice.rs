use core::fmt;

use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::function::Function;
use crate::core::value::Value;

use super::error::arity_error;
use super::sliceable::Sliceable;

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

// call slice
impl Function for Slice {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let start = match self.start {
            Value::Nil => None,
            Value::I64(i) => Some(i),
            _ => return Err(Error::Type(format!("Cannot slice with {}", self.start.type_name(),))),
        };

        let end = match self.end {
            Value::Nil => None,
            Value::I64(i) => Some(i),
            _ => return Err(Error::Type(format!("Cannot slice with {}", self.end.type_name(),))),
        };

        let step = match self.step {
            Value::Nil => None,
            Value::I64(i) => Some(i),
            _ => return Err(Error::Type(format!("Cannot slice with {}", self.step.type_name(),))),
        };

        let result = match &args[0] {
            Value::String(s) => s.slice(start, end, step),
            Value::List(l) => l.slice(start, end, step),
            Value::Vector(v) => v.slice(start, end, step),
            Value::Generator(g) => g.borrow_mut().slice(start, end, step),
            _ => return Err(Error::Type(format!("Cannot slice {}", args[0].type_name(),))),
        };

        result
    }
}
