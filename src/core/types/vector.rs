/* core/types/vector.rs */

use core::fmt;
use std::cell::RefCell;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::core::builtin::generators::EmptyGenerator;
use crate::core::types::error::Result;
use crate::core::types::function::Function;
use crate::core::value::Value;
use crate::core::value::ValueIter;

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Vector {
    pub value: Vec<Value>,
}

impl Vector {
    pub fn new() -> Self {
        Vector {
            value: Vec::<Value>::new(),
        }
    }

    pub fn from(vector: Vec<Value>) -> Self {
        Vector { value: vector }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (n, val) in self.value.iter().enumerate() {
            if n > 0 {
                result += ", ";
            }
            result += format!("{:?}", val).as_str();
        }
        write!(f, "[{}]", result)
    }
}

impl Index<usize> for Vector {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

impl IntoIterator for Vector {
    type Item = Value;
    type IntoIter = ValueIter;

    fn into_iter(self) -> Self::IntoIter {
        ValueIter {
            value: Value::Vector(self),
            current: 0,
            generator: Rc::new(RefCell::new(EmptyGenerator::new())),
        }
    }
}

// set!のことも考えないとな
// def a ([1] [1, 2, 3])したらどうなる?

impl Function for Vector {
    fn call(&self, _args: Vec<Value>) -> Result<Value> {
        Ok(Value::Nil)
    }
}
