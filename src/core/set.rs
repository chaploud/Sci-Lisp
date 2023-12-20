/* set.rs */

use core::fmt;
use std::hash::{Hash, Hasher};

use crate::core::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Set {
    pub value: indexmap::IndexSet<Value>,
}

impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let keys: Vec<_> = self.value.iter().collect();
        Hash::hash(&keys, state);
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:?}", self.value)
    }
}