/* core/types/map.rs */

use core::fmt;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use indexmap::IndexMap;

use crate::core::value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    pub value: indexmap::IndexMap<Value, Value>,
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let pairs: Vec<_> = self.value.iter().collect();
        Hash::hash(&pairs, state);
    }
}

impl Map {
    #[allow(dead_code)]
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

// 辞書の順序付けは、キーの順序付けによって決定される
impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut self_vec: Vec<_> = self.value.iter().collect();
        let mut other_vec: Vec<_> = other.value.iter().collect();
        self_vec.sort();
        other_vec.sort();
        Some(self_vec.cmp(&other_vec))
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_vec: Vec<_> = self.value.iter().collect();
        let mut other_vec: Vec<_> = other.value.iter().collect();
        self_vec.sort();
        other_vec.sort();
        self_vec.cmp(&other_vec)
    }
}

impl Iterator for Map {
    type Item = (Value, Value);

    fn next(&mut self) -> Option<Self::Item> {
        self.value
            .iter()
            .next()
            .map(|(k, v)| (k.clone(), v.clone()))
    }
}
