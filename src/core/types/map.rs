/* core/types/map.rs */

use core::fmt;
use std::hash::{Hash, Hasher};

use indexmap::IndexMap;

use crate::core::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Map{
    pub value: indexmap::IndexMap<Value, Value>,
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let pairs: Vec<_> = self.value.iter().collect();
        Hash::hash(&pairs, state);
    }
}

impl Map {
    pub fn new() -> Self {
        Map {
            value: IndexMap::<Value, Value>::new(),
        }
    }

    pub fn from(vector: Vec<(Value, Value)>) -> Self {
        Map {
            value: vector.into_iter().collect(),
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write the map as a list of key-value pairs
        let mut result = String::new();
        for (n, (key, val)) in self.value.iter().enumerate() {
            if n > 0 {
                result += ", ";
            }
            result += format!("{} {}", key, val).as_str();
        }
        write!(f, "{{{}}}", result)
    }
}
