/* core/types/list.rs */

use core::fmt;
use std::cmp::Ordering;

use crate::core::value::Value;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct List {
    pub value: Vec<Value>,
}

impl List {
    #[allow(dead_code)]
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

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Iterator for List {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.value.iter().next().cloned()
    }
}
