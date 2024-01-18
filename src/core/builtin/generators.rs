use std::fmt;

use crate::core::types::generator::Generator;
use crate::core::types::sliceable::Sliceable;
use crate::core::types::vector::Vector;
use crate::core::value::Value;

// EmptyGenerator
#[derive(Debug, Clone)]
pub struct EmptyGenerator {}

impl EmptyGenerator {
    pub fn new() -> Self {
        EmptyGenerator {}
    }
}

impl fmt::Display for EmptyGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<generator: empty >")
    }
}

impl Iterator for EmptyGenerator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl DoubleEndedIterator for EmptyGenerator {
    fn next_back(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Sliceable for EmptyGenerator {
    fn len(&self) -> usize {
        0
    }
    fn at(&self, _index: i64) -> Option<Value> {
        None
    }
    fn slice(&self, _start: i64, _end: i64, _step: i64) -> Value {
        Value::Nil
    }
}

impl Generator for EmptyGenerator {}

// range
// TODO: Performance and can call multiple times
#[derive(Clone, Debug)]
pub struct Range {
    pub start: i64,
    pub end: i64,
    pub step: i64,
    current: i64,
}

impl Range {
    pub fn new(start: i64, end: i64, step: i64) -> Self {
        Range {
            start,
            end,
            step,
            current: start,
        }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<generator: range ({} {} {}) >",
            self.start, self.end, self.step
        )
    }
}

impl Iterator for Range {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current >= self.end)
            || (self.step < 0 && self.current <= self.end)
        {
            self.current = 0; // HACK: other generator must iterate repeatedly?
            None
        } else {
            let result = self.current;
            self.current += self.step;
            Some(Value::I64(result))
        }
    }
}

// TODO: Maybe bug, repeatly call next_back()
impl DoubleEndedIterator for Range {
    fn next_back(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current >= self.end)
            || (self.step < 0 && self.current <= self.end)
        {
            None
        } else {
            self.current -= self.step;
            Some(Value::I64(self.current))
        }
    }
}

impl Sliceable for Range {
    fn len(&self) -> usize {
        if self.step == 0 {
            0
        } else {
            ((self.end - self.start) / self.step) as usize
        }
    }
    fn at(&self, index: i64) -> Option<Value> {
        if index < 0 {
            let index = self.len() as i64 + index;
            if index < 0 {
                return None;
            }
            Some(Value::I64(self.start + self.step * index))
        } else {
            if index >= self.len() as i64 {
                return None;
            }
            Some(Value::I64(self.start + self.step * index))
        }
    }
    fn slice(&self, start: i64, end: i64, step: i64) -> Value {
        let mut result = Vec::<Value>::new();
        let mut current = start;
        if step > 0 {
            while current < end {
                if let Some(val) = self.at(current) {
                    result.push(val.clone());
                }
                current += step;
            }
        } else {
            while current > end {
                if let Some(val) = self.at(current) {
                    result.push(val.clone());
                }
                current += step;
            }
        }
        Value::Vector(Vector::from(result))
    }
}

impl Generator for Range {}
