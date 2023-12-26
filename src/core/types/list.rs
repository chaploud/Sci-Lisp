/* core/types/list.rs */

use core::fmt;

use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::ifn::IFn;
use crate::core::types::type_name::TypeName;
use crate::core::value::{Evaluable, Value};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct List {
    pub value: Vec<Value>,
}

impl List {
    pub fn new() -> Self {
        List {
            value: Vec::<Value>::new(),
        }
    }

    pub fn from(vector: Vec<Value>) -> Self {
        List { value: vector }
    }
}

impl Evaluable for List {
    fn eval(self, environment: &mut Environment) -> Result<Value> {
        let first = match self.value.first() {
            Some(first) => first,
            None => return Ok(Value::List(self)),
        };
        let rest = &self.value[1..].to_vec();
        match first {
            Value::Symbol(sym) => {
                let ifn = environment.get(&sym.name)?;
                match ifn {
                    Value::Function(f) => f.call(rest.to_vec()),
                    Value::Macro(m) => m.call(rest.to_vec()),
                    _ => return Err(Error::NotCallable(ifn.to_string())),
                }
            }
            Value::Function(f) => f.call(rest.to_vec()),
            Value::Macro(m) => m.call(rest.to_vec()),
            _ => {
                return Err(Error::Type(
                    TypeName::Symbol.to_string(),
                    first.type_name()?.to_string(),
                ))
            }
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = format!("{:?}", self.value);
        result = result[1..result.len() - 1].to_string();
        result = format!("({})", result);
        write!(f, "{}", result)
    }
}
