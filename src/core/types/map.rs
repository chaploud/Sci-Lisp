/* core/types/map.rs */

use core::fmt;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use indexmap::IndexMap;

use crate::core::builtin::generators::EmptyGenerator;
use crate::core::value::Value;
use crate::core::value::ValueIter;

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

impl Default for Map {
    fn default() -> Self {
        Map::new()
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
            result += format!("{:?} {:?}", key, val).as_str();
        }
        write!(f, "{{{}}}", result)
    }
}

// The ordering of dictionaries is determined by the ordering of their keys.
impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

impl Index<usize> for Map {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for Map {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

impl IntoIterator for Map {
    type Item = Value;
    type IntoIter = ValueIter;

    fn into_iter(self) -> Self::IntoIter {
        ValueIter {
            value: Value::Map(self),
            current: 0,
            generator: Rc::new(RefCell::new(EmptyGenerator::new())),
        }
    }
}

impl Map {
    pub fn insert(&mut self, key: Value, value: Value) {
        self.value.insert(key, value);
    }

    pub fn remove(&mut self, key: &Value) -> Option<Value> {
        self.value.remove(key)
    }

    pub fn get(&self, key: &Value) -> Option<&Value> {
        self.value.get(key)
    }
}
