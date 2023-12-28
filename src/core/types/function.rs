/* core/types/function.rs */

use std::fmt;

use crate::core::types::error::Result;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

pub struct Function {
    pub name: Symbol,
    pub func: fn(Vec<Value>) -> Result<Value>,
}

impl Function {
    pub fn call(&self, args: Vec<Value>) -> Result<Value> {
        (self.func)(args)
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function: '{}' ", self.name)
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            name: self.name.clone(),
            func: self.func.clone(),
        }
    }
}

impl std::hash::Hash for Function {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.func.hash(state);
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "function: '{}' ", self.name)
    }
}
