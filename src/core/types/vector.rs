/* core/types/vector.rs */

use core::fmt;
use std::cell::RefCell;
use std::ops::{Index, IndexMut};
use std::rc::Rc;

use crate::core::builtin::generators::EmptyGenerator;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::error::{arity_error, index_out_of_range_error};
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

impl Vector {
    pub fn len(&self) -> usize {
        self.value.len()
    }
    pub fn at(&self, index: i64) -> Option<Value> {
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
    pub fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
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
            result = Self::slice_index(member, result)?;
        }
        Ok(result)
    }

    fn slice_index(member: Value, value: Value) -> Result<Value> {
        match value {
            Value::Vector(vector) => match member {
                Value::Slice(s) => {
                    let mut new_slice = Vec::<Value>::new();
                    let start = match s.start {
                        Value::I64(i) => i,
                        _ => 0,
                    };
                    let end = match s.end {
                        Value::I64(i) => i,
                        _ => vector.len() as i64,
                    };
                    let step = match s.step {
                        Value::I64(i) => i,
                        _ => 1,
                    };
                    let mut current = start;
                    loop {
                        if (step > 0 && current >= end) || (step < 0 && current <= end) {
                            break;
                        }
                        let v = vector.at(current);
                        if v.is_none() {
                            break;
                        }
                        new_slice.push(v.unwrap());
                        current += step;
                    }
                    Value::as_vector(new_slice)
                }
                Value::I64(i) => {
                    let v = vector.at(i);
                    if v.is_none() {
                        return Err(index_out_of_range_error(i));
                    }
                    Ok(v.unwrap())
                }
                _ => Err(Error::Type(
                    "slicing vector can contain only slice or i64".to_string(),
                )),
            },
            _ => Err(Error::Type(
                "slice or index access is allowed only for vector and list".to_string(),
            )),
        }
    }
}

// set!のことも考えないとな
// def a ([1] [1, 2, 3])したらどうなる?
