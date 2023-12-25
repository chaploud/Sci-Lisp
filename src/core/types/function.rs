/* core/types/function.rs */

use std::fmt::{self, Debug};

use crate::core::value::Value;
use crate::core::types::error::Result;
#[derive(Debug, Clone)]
pub struct Function {
    pub name: std::string::String,
    pub call: fn(Vec<Value>) -> Result<Value>,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (function)", self.name)
    }
}
