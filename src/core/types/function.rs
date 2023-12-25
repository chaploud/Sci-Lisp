/* core/types/function.rs */

use std::fmt::{self, Debug};

use crate::core::types::error::Result;
use crate::core::value::Value;
#[derive(Debug, Clone)]
pub struct Function {
    pub name: &'static str,
    pub call: fn(Vec<Value>) -> Result<Value>,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (function)", self.name)
    }
}
