use core::fmt;

use crate::core::value::Value;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Splicing {
    pub value: Vec<Value>,
}

impl Splicing {
    pub fn new() -> Self {
        Splicing {
            value: Vec::<Value>::new(),
        }
    }

    pub fn from(splicing: Vec<Value>) -> Self {
        Splicing { value: splicing }
    }
}

impl Default for Splicing {
    fn default() -> Self {
        Splicing::new()
    }
}

impl fmt::Display for Splicing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "splicing")
    }
}
