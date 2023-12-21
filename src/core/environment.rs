/* environment.rs */

use std::collections::HashMap;

use crate::core::value::Value;
use crate::core::error::Error;

pub type EnvLookup = HashMap<String, Box<Value>>;

#[derive(Debug, PartialEq)]
pub struct Environment<'a> {
    lookup: EnvLookup,
    pub parent: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new(lookup: Option<EnvLookup>, parent: Option<&'a Environment<'a>>) -> Self {
        let mut ret = Self {
            lookup: lookup.unwrap_or_default(),
            parent,
        };

        // TODO: Add core functions to the environment

        ret
    }

    pub fn get(&self, key: &str) -> Result<Box<Value>, Error> {
        match self.lookup.get(key) {
            Some(value) => Ok(value.clone()),
            None => {
                match &self.parent {
                    None => Err(Error::Name(key.to_string())),
                    Some(parent) => parent.get(key),
                }
            },
        }
    }

    pub fn put(&mut self, key: String, value: Box<Value>) {
        let current = self.lookup.entry(key).or_insert_with(|| value.clone());
        if *value != **current {
            *current = value;
        }
    }

}
