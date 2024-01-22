/* core/types/list.rs */

use core::fmt;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::core::builtin::generators::EmptyGenerator;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::sliceable::Sliceable;
use crate::core::value::Value;
use crate::core::value::ValueIter;

use super::sliceable::SliceableMut;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
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

impl Default for List {
    fn default() -> Self {
        List::new()
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (n, val) in self.value.iter().enumerate() {
            if n > 0 {
                result += " ";
            }
            result += format!("{:?}", val).as_str();
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

impl IntoIterator for List {
    type Item = Value;
    type IntoIter = ValueIter;

    fn into_iter(self) -> Self::IntoIter {
        ValueIter {
            value: Value::List(self),
            current: 0,
            generator: Rc::new(RefCell::new(EmptyGenerator::new())),
        }
    }
}

impl Sliceable for List {
    fn len(&self) -> usize {
        self.value.len()
    }
    fn at(&self, index: i64) -> Option<Value> {
        if index < 0 {
            let index = self.len() as i64 + index;
            if index < 0 {
                return None;
            }
            return Some(self.value[index as usize].clone());
        }
        if index as usize >= self.len() {
            return None;
        }
        Some(self.value[index as usize].clone())
    }
    fn slice(&self, start: Option<i64>, end: Option<i64>, step: Option<i64>) -> Result<Value> {
        let mut new_slice = Vec::<Value>::new();

        let step = step.unwrap_or(1);

        if step == 0 {
            return Err(Error::Syntax("step cannot be zero".to_string()));
        }

        if step > 0 {
            let mut start = start.unwrap_or(0);
            let mut end = end.unwrap_or(self.len() as i64);
            if start < 0 {
                start += self.len() as i64;
            }
            if end < 0 {
                end += self.len() as i64;
            }

            start = start.clamp(0, self.len() as i64);
            end = end.clamp(0, self.len() as i64);

            let mut current = start;
            while current < end {
                new_slice.push(self.at(current).unwrap());
                current += step;
            }
        } else {
            let mut start = start.unwrap_or(-1);
            let mut end = end.unwrap_or(-(self.len() as i64) - 1);

            if start > -1 {
                start -= self.len() as i64;
            }
            if end > -1 {
                end -= self.len() as i64;
            }

            start = start.clamp(-(self.len() as i64) - 1, -1);
            end = end.clamp(-(self.len() as i64) - 1, -1);

            let mut current = start;
            while current > end {
                new_slice.push(self.at(current).unwrap());
                current += step;
            }
        }

        Ok(Value::List(List::from(new_slice)))
    }
}

impl SliceableMut for List {
    fn at_mut(&mut self, index: i64) -> Option<&mut Value> {
        if index < 0 {
            let index = self.len() as i64 + index;
            if index < 0 {
                return None;
            }
            return Some(&mut self.value[index as usize]);
        }
        if index as usize >= self.len() {
            return None;
        }
        Some(&mut self.value[index as usize])
    }
}
