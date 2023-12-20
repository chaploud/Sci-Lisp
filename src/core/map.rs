/* map.rs */

use std::hash::{Hash, Hasher};
use core::fmt;

use crate::core::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub value: indexmap::IndexMap<Value, Value>,
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let pairs: Vec<_> = self.value.iter().collect();
        Hash::hash(&pairs, state);
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
