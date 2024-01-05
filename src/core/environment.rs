/* core/environment.rs */

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::builtin::functions::*;
use crate::core::builtin::r#macros::*;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

use super::types::function::Function;
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
        put_builtin_functions(&mut ret);

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

fn put_builtin_functions(env: &mut Environment) {
    let _ = env.put(&TypeFn.name(), Value::Function(Rc::new(TypeFn)));
    let _ = env.put(&PrintFn.name(), Value::Function(Rc::new(PrintFn)));
    let _ = env.put(&AddFn.name(), Value::Function(Rc::new(AddFn)));
    let _ = env.put(&SubFn.name(), Value::Function(Rc::new(SubFn)));
    let _ = env.put(&MulFn.name(), Value::Function(Rc::new(MulFn)));
    let _ = env.put(&DivFn.name(), Value::Function(Rc::new(DivFn)));
    let _ = env.put(&FloorDivFn.name(), Value::Function(Rc::new(FloorDivFn)));
    let _ = env.put(&RemFn.name(), Value::Function(Rc::new(RemFn)));
    let _ = env.put(&EqualFn.name(), Value::Function(Rc::new(EqualFn)));
    let _ = env.put(&NotEqualFn.name(), Value::Function(Rc::new(NotEqualFn)));
    let _ = env.put(&IsFn.name(), Value::Function(Rc::new(IsFn)));
    let _ = env.put(&GeFn.name(), Value::Function(Rc::new(GeFn)));
    let _ = env.put(&GtFn.name(), Value::Function(Rc::new(GtFn)));
    let _ = env.put(&LeFn.name(), Value::Function(Rc::new(LeFn)));
    let _ = env.put(&LtFn.name(), Value::Function(Rc::new(LtFn)));
    let _ = env.put(&DocFn.name(), Value::Function(Rc::new(DocFn)));
}

fn put_builtin_macros(env: &mut Environment) {
    let _ = env.put(&DefMacro.name(), Value::Macro(Rc::new(DefMacro)));
    let _ = env.put(&ConstMacro.name(), Value::Macro(Rc::new(ConstMacro)));
    let _ = env.put(&SetMacro.name(), Value::Macro(Rc::new(SetMacro)));
    let _ = env.put(&LetMacro.name(), Value::Macro(Rc::new(LetMacro)));
    let _ = env.put(&QuoteMacro.name(), Value::Macro(Rc::new(QuoteMacro)));
    let _ = env.put(
        &SyntaxQuoteMacro.name(),
        Value::Macro(Rc::new(SyntaxQuoteMacro)),
    );
    let _ = env.put(&UnquoteMacro.name(), Value::Macro(Rc::new(UnquoteMacro)));
    let _ = env.put(
        &UnquoteSplicingMacro.name(),
        Value::Macro(Rc::new(UnquoteSplicingMacro)),
    );
    let _ = env.put(&DoMacro.name(), Value::Macro(Rc::new(DoMacro)));
    let _ = env.put(&IfMacro.name(), Value::Macro(Rc::new(IfMacro)));
    let _ = env.put(&WhileMacro.name(), Value::Macro(Rc::new(WhileMacro)));
    let _ = env.put(&SwitchMacro.name(), Value::Macro(Rc::new(SwitchMacro)));
    let _ = env.put(&TimeMacro.name(), Value::Macro(Rc::new(TimeMacro)));
}
