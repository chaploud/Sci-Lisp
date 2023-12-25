/* core/types/vector.rs */

use core::fmt;

use crate::core::value::Value;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Vector {
    pub value: Vec<Value>,
}

impl Vector  {
    pub fn new() -> Self {
        Vector {
            value: Vec::<Value>::new(),
        }
    }

    pub fn from(vector: Vec<Value>) -> Self {
        Vector { value: vector }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
