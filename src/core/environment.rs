/* core/environment.rs */

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::core::builtin::functions::*;
use crate::core::builtin::macros::*;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

pub type EnvLookup = HashMap<Symbol, Value>;

#[derive(Debug, PartialEq)]
pub struct Environment<'a> {
    lookup: EnvLookup,
    pub parent: Option<&'a Environment<'a>>,
}

fn put_helper(env: &mut Environment, values: Vec<Value>) {
    for v in values {
        let (key, value) = match v {
            Value::Function(f) => (f.name.clone(), Value::Function(f)),
            Value::Macro(m) => (m.name.clone(), Value::Macro(m)),
            _ => unreachable!(),
        };
        env.put(&key, value).unwrap();
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

    pub fn get(&self, key: &Symbol) -> Result<&Value> {
        match self.lookup.get(key) {
            Some(value) => Ok(value),
            None => match &self.parent {
                None => Err(Error::Name(key.to_string())),
                Some(parent) => parent.get(key),
            },
        }
    }

    pub fn put(&mut self, key: &Symbol, value: Value) -> Result<Value> {
        match self.lookup.entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                if key.meta.mutable {
                    entry.insert(value);
                } else {
                    return Err(Error::Immutable(format!(
                        "cannot overwrite immutable binding '{}'",
                        key
                    )));
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(value);
            }
        };
        Ok(Value::Nil)
    }
}
