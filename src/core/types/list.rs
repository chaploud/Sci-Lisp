/* core/types/list.rs */

use core::fmt;

use crate::core::value::Value;
use crate::core::environment::{Environment, self};

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

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = format!("{:?}", self.value);
        result = result[1..result.len() - 1].to_string();
        result = format!("({})", result);
        write!(f, "{}", result)
    }
}
