/* core/environment.rs */

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::builtin::functions::*;
use crate::core::builtin::r#macros::*;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub root: Rc<RefCell<HashMap<Symbol, Value>>>,
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub current: Option<HashMap<Symbol, Value>>,
}

impl Environment {
    pub fn new_root_environment() -> Rc<RefCell<Self>> {
        let root = Rc::new(RefCell::new(HashMap::new()));
        let result = Rc::new(RefCell::new(Self {
            root: root.clone(),
            parent: None,
            current: None,
        }));

        insert_builtin_macros(&mut *result.borrow_mut());
        insert_builtin_functions(&mut *result.borrow_mut());

        result
    }

    pub fn new_local_environment(parent: &Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        let result = Rc::new(RefCell::new(Self {
            root: parent.borrow().root.clone(),
            parent: Some(parent.clone()),
            current: Some(HashMap::new()),
        }));
        result
    }

    pub fn get(&self, key: Symbol) -> Result<(Symbol, Value)> {
        match self.current.clone().unwrap_or_default().get_key_value(&key) {
            Some((key, val)) => Ok((key.clone(), val.clone())),
            None => match &self.parent {
                None => Err(Error::Name(key.to_string())),
                Some(parent) => parent.borrow().get(key),
            },
        }
    }

    pub fn insert_to_root(&mut self, key: Symbol, value: Value) -> Result<Value> {
        match self.root.clone().borrow_mut().entry(key.clone()) {
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

    pub fn insert_to_current(&mut self, key: Symbol, value: Value) -> Result<Value> {
        match self.current.clone().unwrap_or_default().entry(key.clone()) {
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

    pub fn set(&mut self, key: &Symbol, value: Value) -> Result<Value> {
        match self.current.clone().unwrap_or_default().entry(key.clone()) {
            Entry::Occupied(mut entry) => {
                if entry.key().meta.mutable {
                    entry.insert(value);
                } else {
                    return Err(Error::Immutable(format!(
                        "cannot overwrite immutable binding '{}'",
                        key
                    )));
                }
            }
            // 親環境までたどってsetを試みる。できなかったらエラー
            Entry::Vacant(_) => match self.parent.clone() {
                None => return Err(Error::Name(key.to_string())),
                Some(parent) => {
                    parent.borrow_mut().set(key, value)?;
                    return Ok(Value::Nil);
                }
            },
        };
        Ok(Value::Nil)
    }
}

fn insert_builtin_functions(env: &mut Environment) {
    let _ = env.insert_to_root(SYMBOL_TYPE, Value::Function(Rc::new(TypeFn)));
    let _ = env.insert_to_root(SYMBOL_PRINT, Value::Function(Rc::new(PrintFn)));
    let _ = env.insert_to_root(SYMBOL_ADD, Value::Function(Rc::new(AddFn)));
    let _ = env.insert_to_root(SYMBOL_SUB, Value::Function(Rc::new(SubFn)));
    let _ = env.insert_to_root(SYMBOL_MUL, Value::Function(Rc::new(MulFn)));
    let _ = env.insert_to_root(SYMBOL_DIV, Value::Function(Rc::new(DivFn)));
    let _ = env.insert_to_root(SYMBOL_FLOORDIV, Value::Function(Rc::new(FloorDivFn)));
    let _ = env.insert_to_root(SYMBOL_REM, Value::Function(Rc::new(RemFn)));
    let _ = env.insert_to_root(SYMBOL_EQUAL, Value::Function(Rc::new(EqualFn)));
    let _ = env.insert_to_root(SYMBOL_NOTEQUAL, Value::Function(Rc::new(NotEqualFn)));
    let _ = env.insert_to_root(SYMBOL_IS, Value::Function(Rc::new(IsFn)));
    let _ = env.insert_to_root(SYMBOL_GE, Value::Function(Rc::new(GeFn)));
    let _ = env.insert_to_root(SYMBOL_GT, Value::Function(Rc::new(GtFn)));
    let _ = env.insert_to_root(SYMBOL_LE, Value::Function(Rc::new(LeFn)));
    let _ = env.insert_to_root(SYMBOL_LT, Value::Function(Rc::new(LtFn)));
    let _ = env.insert_to_root(SYMBOL_STR, Value::Function(Rc::new(StrFn)));
    let _ = env.insert_to_root(SYMBOL_I64, Value::Function(Rc::new(I64Fn)));
    let _ = env.insert_to_root(SYMBOL_F64, Value::Function(Rc::new(F64Fn)));
    let _ = env.insert_to_root(SYMBOL_FIRST, Value::Function(Rc::new(FirstFn)));
    let _ = env.insert_to_root(SYMBOL_REST, Value::Function(Rc::new(RestFn)));
}

fn insert_builtin_macros(env: &mut Environment) {
    let _ = env.insert_to_root(SYMBOL_DEF, Value::Macro(Rc::new(DefMacro)));
    let _ = env.insert_to_root(SYMBOL_CONST, Value::Macro(Rc::new(ConstMacro)));
    let _ = env.insert_to_root(SYMBOL_SET, Value::Macro(Rc::new(SetMacro)));
    let _ = env.insert_to_root(SYMBOL_LET, Value::Macro(Rc::new(LetMacro)));
    let _ = env.insert_to_root(SYMBOL_QUOTE, Value::Macro(Rc::new(QuoteMacro)));
    let _ = env.insert_to_root(SYMBOL_SYNTAX_QUOTE, Value::Macro(Rc::new(SyntaxQuoteMacro)));
    let _ = env.insert_to_root(SYMBOL_UNQUOTE, Value::Macro(Rc::new(UnquoteMacro)));
    let _ = env.insert_to_root(
        SYMBOL_UNQUOTE_SPLICING,
        Value::Macro(Rc::new(UnquoteSplicingMacro)),
    );
    let _ = env.insert_to_root(SYMBOL_DO, Value::Macro(Rc::new(DoMacro)));
    let _ = env.insert_to_root(SYMBOL_IF, Value::Macro(Rc::new(IfMacro)));
    let _ = env.insert_to_root(SYMBOL_WHILE, Value::Macro(Rc::new(WhileMacro)));
    let _ = env.insert_to_root(SYMBOL_SWITCH, Value::Macro(Rc::new(SwitchMacro)));
    let _ = env.insert_to_root(SYMBOL_TIME, Value::Macro(Rc::new(TimeMacro)));
    let _ = env.insert_to_root(SYMBOL_DOC, Value::Macro(Rc::new(DocMacro)));
    let _ = env.insert_to_root(SYMBOL_FN, Value::Macro(Rc::new(FnMacro)));
}
