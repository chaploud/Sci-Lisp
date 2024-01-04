/* core/types/list.rs */

use core::fmt;
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

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
        let mut result = String::new();
        for (n, val) in self.value.iter().enumerate() {
            if n > 0 {
                result += " ";
            }
            result += format!("{}", val).as_str();
        }
        write!(f, "({})", result)
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

impl Index<usize> for List {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for List {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}
