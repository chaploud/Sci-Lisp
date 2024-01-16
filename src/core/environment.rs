/* core/environment.rs */

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;

use nohash::BuildNoHashHasher;

use crate::core::builtin::functions::*;
use crate::core::builtin::r#macros::*;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

type Lookup = HashMap<Symbol, Value, BuildNoHashHasher<usize>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    pub root: Rc<RefCell<Lookup>>,
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub current: Option<Rc<RefCell<Lookup>>>,
}

impl Environment {
    pub fn new_root_environment() -> Rc<RefCell<Self>> {
        let root: Rc<RefCell<Lookup>> = Rc::new(RefCell::new(HashMap::with_hasher(
            BuildNoHashHasher::default(),
        )));
        let result = Rc::new(RefCell::new(Self {
            root,
            parent: None,
            current: None,
        }));

        insert_builtin_macros(&mut result.borrow_mut());
        insert_builtin_functions(&mut result.borrow_mut());

        result
    }

    pub fn new_local_environment(parent: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        let result = Rc::new(RefCell::new(Self {
            root: parent.borrow_mut().root.clone(),
            parent: Some(parent.clone()),
            current: Some(Rc::new(RefCell::new(HashMap::with_hasher(
                BuildNoHashHasher::default(),
            )))),
        }));
        result
    }

    pub fn get(&self, key: Symbol) -> Result<(Symbol, Value)> {
        match self
            .current
            .clone()
            .unwrap_or_default()
            .borrow()
            .get_key_value(&key)
        {
            Some((key, val)) => Ok((key.clone(), val.clone())),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(key),
                None => match self.root.borrow().get_key_value(&key) {
                    Some((k, v)) => Ok((k.clone(), v.clone())),
                    None => Err(Error::Name(key.to_string())),
                },
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
        match self
            .current
            .clone()
            .unwrap_or_default()
            .borrow_mut()
            .entry(key.clone())
        {
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
        match self
            .current
            .clone()
            .unwrap_or_default()
            .borrow_mut()
            .entry(key.clone())
        {
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
            Entry::Vacant(_) => match self.parent.clone() {
                Some(parent) => {
                    parent.borrow_mut().set(key, value)?;
                    return Ok(Value::Nil);
                }
                None => match self.root.clone().borrow_mut().entry(key.clone()) {
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
                    Entry::Vacant(_) => {
                        return Err(Error::Name(key.to_string()));
                    }
                },
            },
        };
        Ok(Value::Nil)
    }
}

fn insert_builtin_functions(env: &mut Environment) {
    let _ = env.insert_to_root(SYMBOL_TYPE, Value::Function(Rc::new(RefCell::new(TypeFn))));
    let _ = env.insert_to_root(
        SYMBOL_PRINT,
        Value::Function(Rc::new(RefCell::new(PrintFn))),
    );
    let _ = env.insert_to_root(SYMBOL_ADD, Value::Function(Rc::new(RefCell::new(AddFn))));
    let _ = env.insert_to_root(SYMBOL_SUB, Value::Function(Rc::new(RefCell::new(SubFn))));
    let _ = env.insert_to_root(SYMBOL_MUL, Value::Function(Rc::new(RefCell::new(MulFn))));
    let _ = env.insert_to_root(SYMBOL_DIV, Value::Function(Rc::new(RefCell::new(DivFn))));
    let _ = env.insert_to_root(
        SYMBOL_FLOORDIV,
        Value::Function(Rc::new(RefCell::new(FloorDivFn))),
    );
    let _ = env.insert_to_root(SYMBOL_REM, Value::Function(Rc::new(RefCell::new(RemFn))));
    let _ = env.insert_to_root(
        SYMBOL_EQUAL,
        Value::Function(Rc::new(RefCell::new(EqualFn))),
    );
    let _ = env.insert_to_root(
        SYMBOL_NOTEQUAL,
        Value::Function(Rc::new(RefCell::new(NotEqualFn))),
    );
    let _ = env.insert_to_root(SYMBOL_IS, Value::Function(Rc::new(RefCell::new(IsFn))));
    let _ = env.insert_to_root(SYMBOL_GE, Value::Function(Rc::new(RefCell::new(GeFn))));
    let _ = env.insert_to_root(SYMBOL_GT, Value::Function(Rc::new(RefCell::new(GtFn))));
    let _ = env.insert_to_root(SYMBOL_LE, Value::Function(Rc::new(RefCell::new(LeFn))));
    let _ = env.insert_to_root(SYMBOL_LT, Value::Function(Rc::new(RefCell::new(LtFn))));
    let _ = env.insert_to_root(SYMBOL_STR, Value::Function(Rc::new(RefCell::new(StrFn))));
    let _ = env.insert_to_root(SYMBOL_I64, Value::Function(Rc::new(RefCell::new(I64Fn))));
    let _ = env.insert_to_root(SYMBOL_F64, Value::Function(Rc::new(RefCell::new(F64Fn))));
    let _ = env.insert_to_root(
        SYMBOL_FIRST,
        Value::Function(Rc::new(RefCell::new(FirstFn))),
    );
    let _ = env.insert_to_root(SYMBOL_REST, Value::Function(Rc::new(RefCell::new(RestFn))));
    let _ = env.insert_to_root(
        SYMBOL_RANGE,
        Value::Function(Rc::new(RefCell::new(RangeFn))),
    );
    let _ = env.insert_to_root(
        SYMBOL_GENSYM,
        Value::Function(Rc::new(RefCell::new(GensymFn { id: 0 }))),
    );
}

fn insert_builtin_macros(env: &mut Environment) {
    let _ = env.insert_to_root(SYMBOL_DEF, Value::Macro(Rc::new(RefCell::new(DefMacro))));
    let _ = env.insert_to_root(
        SYMBOL_CONST,
        Value::Macro(Rc::new(RefCell::new(ConstMacro))),
    );
    let _ = env.insert_to_root(SYMBOL_SET, Value::Macro(Rc::new(RefCell::new(SetMacro))));
    let _ = env.insert_to_root(SYMBOL_LET, Value::Macro(Rc::new(RefCell::new(LetMacro))));
    let _ = env.insert_to_root(
        SYMBOL_QUOTE,
        Value::Macro(Rc::new(RefCell::new(QuoteMacro))),
    );
    let _ = env.insert_to_root(
        SYMBOL_SYNTAX_QUOTE,
        Value::Macro(Rc::new(RefCell::new(SyntaxQuoteMacro))),
    );
    let _ = env.insert_to_root(SYMBOL_DO, Value::Macro(Rc::new(RefCell::new(DoMacro))));
    let _ = env.insert_to_root(SYMBOL_IF, Value::Macro(Rc::new(RefCell::new(IfMacro))));
    let _ = env.insert_to_root(
        SYMBOL_WHILE,
        Value::Macro(Rc::new(RefCell::new(WhileMacro))),
    );
    let _ = env.insert_to_root(
        SYMBOL_SWITCH,
        Value::Macro(Rc::new(RefCell::new(SwitchMacro))),
    );
    let _ = env.insert_to_root(SYMBOL_TIME, Value::Macro(Rc::new(RefCell::new(TimeMacro))));
    let _ = env.insert_to_root(SYMBOL_DOC, Value::Macro(Rc::new(RefCell::new(DocMacro))));
    let _ = env.insert_to_root(SYMBOL_FN, Value::Macro(Rc::new(RefCell::new(FnMacro))));
    let _ = env.insert_to_root(SYMBOL_DEFN, Value::Macro(Rc::new(RefCell::new(DefnMacro))));
    let _ = env.insert_to_root(
        SYMBOL_THREAD_FIRST,
        Value::Macro(Rc::new(RefCell::new(ThreadFirstMacro))),
    );
    let _ = env.insert_to_root(
        SYMBOL_THREAD_LAST,
        Value::Macro(Rc::new(RefCell::new(ThreadLastMacro))),
    );
    let _ = env.insert_to_root(SYMBOL_COND, Value::Macro(Rc::new(RefCell::new(CondMacro))));
    let _ = env.insert_to_root(SYMBOL_AND, Value::Macro(Rc::new(RefCell::new(AndMacro))));
    let _ = env.insert_to_root(SYMBOL_OR, Value::Macro(Rc::new(RefCell::new(OrMacro))));
    let _ = env.insert_to_root(SYMBOL_FOR, Value::Macro(Rc::new(RefCell::new(ForMacro))));
    let _ = env.insert_to_root(
        SYMBOL_MACRO,
        Value::Macro(Rc::new(RefCell::new(MacroMacro))),
    );
}
