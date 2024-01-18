/* core/types/vector.rs */

use core::fmt;
use std::cell::RefCell;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::core::builtin::generators::EmptyGenerator;
use crate::core::types::error::arity_error;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::sliceable::Sliceable;
use crate::core::value::Value;
use crate::core::value::ValueIter;

use super::function::Function;

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

impl Default for Vector {
    fn default() -> Self {
        Vector::new()
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

impl Sliceable for Vector {
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

        Ok(Value::Vector(Vector::from(new_slice)))
    }
}

impl Vector {
    pub fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        if self.value.is_empty() {
            return Err(Error::Syntax("cannot call empty vector".to_string()));
        }

        for member in self.value.clone() {
            match member {
                Value::Slice(_) | Value::I64(_) => {}
                _ => {
                    return Err(Error::Type(
                        "slicing vector can contain only slice or i64".to_string(),
                    ))
                }
            }
        }

        let mut result = args[0].clone();
        for member in self.value.clone() {
            match member {
                Value::Slice(slice) => {
                    result = slice.call(vec![result])?;
                }
                Value::I64(i) => {
                    result = i.call(vec![result])?;
                }
                _ => unreachable!(),
            }
        }
        Ok(result)
    }
}
