use std::fmt;

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

impl Generator for EmptyGenerator {
    fn can_reverse(&self) -> bool {
        true
    }
}

// range
#[derive(Clone)]
pub struct Range {
    start: i64,
    end: i64,
    step: i64,
}

impl Range {
    pub fn new(start: i64, end: i64, step: i64) -> Self {
        Range { start, end, step }
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
        if self.start < self.end {
            let result = self.start;
            self.start += self.step;
            Some(Value::I64(result))
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for Range {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= self.step;
            Some(Value::I64(self.end))
        } else {
            None
        }
    }
}

impl Generator for Range {
    fn can_reverse(&self) -> bool {
        true
    }
}
