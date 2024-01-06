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

    pub fn get(&self, key: &Symbol) -> Result<(&Symbol, &Value)> {
        match self.lookup.get_key_value(key) {
            Some((key, val)) => Ok((key, val)),
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
    let _ = env.put(&SYMBOL_TYPE, Value::Function(Rc::new(TypeFn)));
    let _ = env.put(&SYMBOL_PRINT, Value::Function(Rc::new(PrintFn)));
    let _ = env.put(&SYMBOL_ADD, Value::Function(Rc::new(AddFn)));
    let _ = env.put(&SYMBOL_SUB, Value::Function(Rc::new(SubFn)));
    let _ = env.put(&SYMBOL_MUL, Value::Function(Rc::new(MulFn)));
    let _ = env.put(&SYMBOL_DIV, Value::Function(Rc::new(DivFn)));
    let _ = env.put(&SYMBOL_FLOORDIV, Value::Function(Rc::new(FloorDivFn)));
    let _ = env.put(&SYMBOL_REM, Value::Function(Rc::new(RemFn)));
    let _ = env.put(&SYMBOL_EQUAL, Value::Function(Rc::new(EqualFn)));
    let _ = env.put(&SYMBOL_NOTEQUAL, Value::Function(Rc::new(NotEqualFn)));
    let _ = env.put(&SYMBOL_IS, Value::Function(Rc::new(IsFn)));
    let _ = env.put(&SYMBOL_GE, Value::Function(Rc::new(GeFn)));
    let _ = env.put(&SYMBOL_GT, Value::Function(Rc::new(GtFn)));
    let _ = env.put(&SYMBOL_LE, Value::Function(Rc::new(LeFn)));
    let _ = env.put(&SYMBOL_LT, Value::Function(Rc::new(LtFn)));
    let _ = env.put(&SYMBOL_STR, Value::Function(Rc::new(StrFn)));
    let _ = env.put(&SYMBOL_I64, Value::Function(Rc::new(I64Fn)));
    let _ = env.put(&SYMBOL_F64, Value::Function(Rc::new(F64Fn)));
    let _ = env.put(&SYMBOL_FIRST, Value::Function(Rc::new(FirstFn)));
}

fn put_builtin_macros(env: &mut Environment) {
    let _ = env.put(&SYMBOL_DEF, Value::Macro(Rc::new(DefMacro)));
    let _ = env.put(&SYMBOL_CONST, Value::Macro(Rc::new(ConstMacro)));
    let _ = env.put(&SYMBOL_SET, Value::Macro(Rc::new(SetMacro)));
    let _ = env.put(&SYMBOL_LET, Value::Macro(Rc::new(LetMacro)));
    let _ = env.put(&SYMBOL_QUOTE, Value::Macro(Rc::new(QuoteMacro)));
    let _ = env.put(
        &SYMBOL_SYNTAX_QUOTE,
        Value::Macro(Rc::new(SyntaxQuoteMacro)),
    );
    let _ = env.put(&SYMBOL_UNQUOTE, Value::Macro(Rc::new(UnquoteMacro)));
    let _ = env.put(
        &SYMBOL_UNQUOTE_SPLICING,
        Value::Macro(Rc::new(UnquoteSplicingMacro)),
    );
    let _ = env.put(&SYMBOL_DO, Value::Macro(Rc::new(DoMacro)));
    let _ = env.put(&SYMBOL_IF, Value::Macro(Rc::new(IfMacro)));
    let _ = env.put(&SYMBOL_WHILE, Value::Macro(Rc::new(WhileMacro)));
    let _ = env.put(&SYMBOL_SWITCH, Value::Macro(Rc::new(SwitchMacro)));
    let _ = env.put(&SYMBOL_TIME, Value::Macro(Rc::new(TimeMacro)));
    let _ = env.put(&SYMBOL_DOC, Value::Macro(Rc::new(DocMacro)));
}
