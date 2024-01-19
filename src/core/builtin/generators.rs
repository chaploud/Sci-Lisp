use std::fmt;

use crate::core::types::error::Error;
use crate::core::types::error::Result;
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
    fn slice(&self, _start: Option<i64>, _end: Option<i64>, _step: Option<i64>) -> Result<Value> {
        Ok(Value::Nil)
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
        write!(f, "<generator: range ({} {} {}) >", self.start, self.end, self.step)
    }
}

impl Iterator for Range {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current >= self.end) || (self.step < 0 && self.current <= self.end) {
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
        if (self.step > 0 && self.current >= self.end) || (self.step < 0 && self.current <= self.end) {
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

impl Generator for Range {}
