/* core/environment.rs */

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::core::builtin::functions::*;
use crate::core::builtin::r#macros::*;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

use super::types::r#macro::Macro;

pub type EnvLookup = HashMap<Symbol, Value>;

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

        put_builtin_macros(&mut ret);
        put_builtin_functions(&mut ret, ALL_FUNCTIONS.to_vec());

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
                if entry.key().meta.mutable {
                    if !key.meta.mutable {
                        // overwrite with const
                        return Err(Error::Immutable(format!(
                            "cannot overwrite '{}' with const",
                            key
                        )));
                    }
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

fn put_builtin_functions(env: &mut Environment, values: Vec<Value>) {
    for v in values {
        let (key, value) = match v {
            Value::Function(f) => (f.name.clone(), Value::Function(f)),
            _ => unreachable!(),
        };
        env.put(&key, value).unwrap();
    }
}

fn put_builtin_macros(env: &mut Environment) {
    let def_macro = DefMacro {};
    let const_macro = ConstMacro {};
    let set_macro = SetMacro {};
    let let_macro = LetMacro {};
    let quote_macro = QuoteMacro {};
    let syntax_quote_macro = SyntaxQuoteMacro {};
    let unquote_macro = UnquoteMacro {};
    let unquote_splicing_macro = UnquoteSplicingMacro {};
    let do_macro = DoMacro {};
    let if_macro = IfMacro {};
    let while_macro = WhileMacro {};
    let switch_macro = SwitchMacro {};
    let time_macro = TimeMacro {};

    env.put(&def_macro.name(), def_macro);
    env.put(&const_macro.name(), const_macro);
    env.put(&set_macro.name(), set_macro);
    env.put(&let_macro.name(), let_macro);
    env.put(&quote_macro.name(), quote_macro);
    env.put(&syntax_quote_macro.name(), syntax_quote_macro);
    env.put(&unquote_macro.name(), unquote_macro);
    env.put(&unquote_splicing_macro.name(), unquote_splicing_macro);
    env.put(&do_macro.name(), do_macro);
    env.put(&if_macro.name(), if_macro);
    env.put(&while_macro.name(), while_macro);
    env.put(&switch_macro.name(), switch_macro);
    env.put(&time_macro.name(), time_macro);
}
