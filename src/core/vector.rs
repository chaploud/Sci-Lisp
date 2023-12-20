/* vector.rs */

use core::fmt;

use crate::core::value::Value;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Vector {
    pub value: Vec<Value>,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
