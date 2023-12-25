/* core/types/macro.rs */

use std::fmt::{self, Debug};

use crate::core::types::error::Result;
use crate::core::value::Value;
#[derive(Debug, Clone)]
pub struct Macro {
    pub name: &'static str,
    pub call: fn(Vec<Value>) -> Result<Value>,
}

impl fmt::Display for Macro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (macro)", self.name)
    }
}
