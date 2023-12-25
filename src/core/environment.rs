/* core/environment.rs */

use std::collections::HashMap;

use crate::core::builtin::functions::*;
use crate::core::builtin::macros::*;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::value::Value;

pub type EnvLookup = HashMap<String, Value>;

#[derive(Debug, PartialEq)]
pub struct Environment<'a> {
    lookup: EnvLookup,
    pub parent: Option<&'a Environment<'a>>,
}

fn put_helper(env: &mut Environment, values: Vec<Value>) {
    for v in values {
        let (key, value) = match v {
            Value::Function(f) => (f.name.to_string(), Value::Function(f)),
            Value::Macro(m) => (m.name.to_string(), Value::Macro(m)),
            _ => unreachable!(),
        };
        env.put(key, value).unwrap();
    }
}

impl<'a> Environment<'a> {
    pub fn new(lookup: Option<EnvLookup>, parent: Option<&'a Environment<'a>>) -> Self {
        let mut ret = Self {
            lookup: lookup.unwrap_or_default(),
            parent,
        };

        put_helper(&mut ret, ALL_FUNCTIONS.to_vec());
        put_helper(&mut ret, ALL_MACROS.to_vec());

        ret
    }

    pub fn get(&self, key: &str) -> Result<&Value> {
        match self.lookup.get(key) {
            Some(value) => Ok(value),
            None => match &self.parent {
                None => Err(Error::Name(key.to_string())),
                Some(parent) => parent.get(key),
            },
        }
    }

    pub fn put(&mut self, key: String, mut value: Value) -> Result<Value> {
        let mut current = self.lookup.entry(key).or_insert_with(|| value.clone());
        if value != *current {
            current = &mut value;
        }
        Ok(Value::Nil)
    }
}
