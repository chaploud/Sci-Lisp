use std::fmt;
use std::ops::Index;

use crate::core::types::generator::Generator;
use crate::core::value::Value;

// EmptyGenerator
#[derive(Clone)]
pub struct EmptyGenerator {}

impl EmptyGenerator {
    pub fn new() -> Self {
        EmptyGenerator {}
    }
}

impl fmt::Display for EmptyGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "empty_generator()")
    }
}

impl fmt::Debug for EmptyGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "empty_generator()")
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

impl Index<usize> for EmptyGenerator {
    type Output = Value;

    fn index(&self, _index: usize) -> &Self::Output {
        &Value::Nil
    }
}

impl ExactSizeIterator for EmptyGenerator {
    fn len(&self) -> usize {
        0
    }
}

impl Generator for EmptyGenerator {
    fn can_reverse(&self) -> bool {
        true
    }
    fn at(&self, _index: i64) -> Option<Value> {
        Some(Value::Nil)
    }
}

// range
// TODO: Performance and can call multiple times
#[derive(Clone)]
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
        write!(f, "range({}, {}, {})", self.start, self.end, self.step)
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "range({}, {}, {})", self.start, self.end, self.step)
    }
}

impl Iterator for Range {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current >= self.end)
            || (self.step < 0 && self.current <= self.end)
        {
            None
        } else {
            let result = self.current;
            self.current += self.step;
            Some(Value::I64(result))
        }
    }
}

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

impl ExactSizeIterator for Range {
    fn len(&self) -> usize {
        if self.step == 0 {
            0
        } else {
            ((self.end - self.start) / self.step) as usize
        }
    }
}

impl Generator for Range {
    fn can_reverse(&self) -> bool {
        true
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
}
