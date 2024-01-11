/* core/types/set.rs */

use core::fmt;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::rc::Rc;

use indexmap::IndexSet;

use crate::core::builtin::generators::EmptyGenerator;
use crate::core::value::Value;
use crate::core::value::ValueIter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Set {
    pub value: indexmap::IndexSet<Value>,
}

impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let keys: Vec<_> = self.value.iter().collect();
        Hash::hash(&keys, state);
    }
}

impl Set {
    pub fn new() -> Self {
        Set {
            value: IndexSet::<Value>::new(),
        }
    }

    pub fn from(vector: Vec<Value>) -> Self {
        Set {
            value: vector.into_iter().collect(),
        }
    }
}

impl PartialOrd for Set {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut self_vec: Vec<_> = self.value.iter().collect();
        let mut other_vec: Vec<_> = other.value.iter().collect();
        self_vec.sort();
        other_vec.sort();
        Some(self_vec.cmp(&other_vec))
    }
}

impl Ord for Set {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_vec: Vec<_> = self.value.iter().collect();
        let mut other_vec: Vec<_> = other.value.iter().collect();
        self_vec.sort();
        other_vec.sort();
        self_vec.cmp(&other_vec)
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (n, val) in self.value.iter().enumerate() {
            if n > 0 {
                result += ", ";
            }
            result += format!("{:?}", val).as_str();
        }
        write!(f, "#{{{}}}", result)
    }
}

impl Index<usize> for Set {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IntoIterator for Set {
    type Item = Value;
    type IntoIter = ValueIter;

    fn into_iter(self) -> Self::IntoIter {
        ValueIter {
            value: Value::Set(self),
            current: 0,
            generator: Rc::new(RefCell::new(EmptyGenerator::new())),
        }
    }
}
