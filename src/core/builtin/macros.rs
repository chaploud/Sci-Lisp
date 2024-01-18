/* core/builtin/macros.rs */

use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::ops::ControlFlow;
use std::rc::Rc;
use std::vec;

use once_cell::sync::Lazy;

use crate::core::environment::Environment;
use crate::core::eval::eval;
use crate::core::types::error::type_error;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::error::{arity_error, arity_error_min, arity_error_range};
use crate::core::types::keyword::Keyword;
use crate::core::types::lambda::Lambda;
use crate::core::types::meta::Meta;
use crate::core::types::r#macro::ControlFlowMacro;
use crate::core::types::r#macro::Macro;
use crate::core::types::r#macro::SplicingMacro;
use crate::core::types::symbol::Symbol;
use crate::core::types::vector::Vector;
use crate::core::value::Value;

// def
pub static SYMBOL_DEF: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("def"),
    meta: Meta {
        doc: Cow::Borrowed("Bind a value to a symbol."),
        mutable: false,
    },
    hash: fxhash::hash("def"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefMacro;

impl Macro for DefMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() || args.len() > 3 {
            return Err(arity_error_range(1, 3, args.len()));
        }

        let mut symbol = match args[0].clone() {
            Value::Symbol(sym) => sym,
            _ => {
                return Err(Error::Type(
                    "def: first argument must be a symbol".to_string(),
                ))
            }
        };
        let body: Value;

        if args.len() == 1 {
            body = Value::Nil;
        } else if args.len() == 2 {
            body = args[1].clone();
        } else {
            let docstring = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::Type("def: docstring must be a string".to_string())),
            };

            symbol.meta.doc = Cow::Owned(docstring);
            body = args[2].clone();
        }

        let value = eval(body, environment.clone())?;

        environment.borrow_mut().insert(&symbol, value)?;

        Ok(Value::Symbol(symbol))
    }
}

impl fmt::Display for DefMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: def>")
    }
}

// const
pub static SYMBOL_CONST: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("const"),
    meta: Meta {
        doc: Cow::Borrowed("Bind a value to a symbol."),
        mutable: false,
    },
    hash: fxhash::hash("const"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstMacro;

impl Macro for ConstMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() || args.len() > 3 {
            return Err(arity_error_range(1, 3, args.len()));
        }

        let mut symbol = match args[0].clone() {
            Value::Symbol(sym) => sym,
            _ => {
                return Err(Error::Type(
                    "const: first argument must be a symbol".to_string(),
                ))
            }
        };
        let body: Value;

        if args.len() == 1 {
            body = Value::Nil;
        } else if args.len() == 2 {
            body = args[1].clone();
        } else {
            let docstring = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::Type("def: docstring must be a string".to_string())),
            };

            symbol.meta.doc = Cow::Owned(docstring);
            body = args[2].clone();
        }

        let value = eval(body, environment.clone())?;

        symbol.meta.mutable = false;

        environment.borrow_mut().insert(&symbol, value)?;

        Ok(Value::Symbol(symbol))
    }
}

impl fmt::Display for ConstMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: const>")
    }
}

// set!
pub static SYMBOL_SET: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("set!"),
    meta: Meta {
        doc: Cow::Borrowed("Bind a value to a symbol."),
        mutable: false,
    },
    hash: fxhash::hash("set!"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetMacro;

impl Macro for SetMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }

        let mut args_for_set: Vec<Value> = vec![];
        for (i, v) in args.into_iter().enumerate() {
            if i == 0 {
                args_for_set.push(v);
            } else {
                args_for_set.push(eval(v, environment.clone())?);
            }
        }

        let symbol = match args_for_set[0].clone() {
            Value::Symbol(sym) => sym,
            _ => {
                return Err(Error::Type(
                    "set: first argument must be a symbol".to_string(),
                ))
            }
        };

        let value = args_for_set[1].clone();

        environment.borrow_mut().set(&symbol, value)?;
        Ok(Value::Symbol(symbol))
    }
}

impl fmt::Display for SetMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: set!>")
    }
}

// let
pub static SYMBOL_LET: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("let"),
    meta: Meta {
        doc: Cow::Borrowed("Bind a value to a symbol in a local scope."),
        mutable: false,
    },
    hash: fxhash::hash("let"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetMacro;

impl Macro for LetMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }

        let local_env = Environment::new_local_environment(environment);

        let bind_form = match &args[0] {
            Value::Vector(v) => v,
            _ => {
                return Err(Error::Syntax(
                    "let: first argument must be a vector".to_string(),
                ))
            }
        };

        if bind_form.value.len() % 2 != 0 {
            return Err(Error::Syntax(
                "let: first argument must be a vector of even length".to_string(),
            ));
        }

        for pair in bind_form.value.chunks(2) {
            let key = match &pair[0] {
                Value::Symbol(sym) => Ok(sym),
                _ => Err(Error::Type(
                    "let: first element of each pair must be a symbol".to_string(),
                )),
            };

            let val = pair[1].clone();

            local_env.borrow_mut().insert(key?, val)?;
        }

        let mut result = Value::Nil;
        for arg in args.into_iter().skip(1) {
            result = eval(arg, local_env.clone())?
        }

        Ok(result)
    }
}

impl fmt::Display for LetMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: let>")
    }
}

// quote(')
pub static SYMBOL_QUOTE: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("quote"),
    meta: Meta {
        doc: Cow::Borrowed("Quote a value."),
        mutable: false,
    },
    hash: fxhash::hash("quote"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuoteMacro;

impl Macro for QuoteMacro {
    fn call(&self, args: Vec<Value>, _environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Ok(args[0].clone())
    }
}

impl fmt::Display for QuoteMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: quote(')>")
    }
}

// syntax-quote(`)
pub static SYMBOL_SYNTAX_QUOTE: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("syntax-quote"),
    meta: Meta {
        doc: Cow::Borrowed("Syntax-quote a value."),
        mutable: false,
    },
    hash: fxhash::hash("syntax-quote"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxQuoteMacro;

impl Macro for SyntaxQuoteMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let local_env = Environment::new_local_environment(environment.clone());
        local_env
            .borrow_mut()
            .insert(&SYMBOL_UNQUOTE, Value::Macro(Rc::new(UnquoteMacro)))?;
        local_env.borrow_mut().insert(
            &SYMBOL_UNQUOTE_SPLICING,
            Value::SplicingMacro(Rc::new(UnquoteSplicingMacro)),
        )?;

        eval(args[0].clone(), local_env)
    }
}

impl fmt::Display for SyntaxQuoteMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: syntax-quote(`)>")
    }
}

// unquote(~)
pub static SYMBOL_UNQUOTE: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("unquote"),
    meta: Meta {
        doc: Cow::Borrowed("Unquote a value."),
        mutable: false,
    },
    hash: fxhash::hash("unquote"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnquoteMacro;

impl Macro for UnquoteMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        eval(args[0].clone(), environment)
    }
}

impl fmt::Display for UnquoteMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: unquote(~)>")
    }
}

// unquote-splicing(~@)
pub static SYMBOL_UNQUOTE_SPLICING: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("unquote-splicing"),
    meta: Meta {
        doc: Cow::Borrowed("Unquote-splicing a value."),
        mutable: false,
    },
    hash: fxhash::hash("unquote-splicing"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnquoteSplicingMacro;

// NOTE: impl 'SplicingMacro'
impl SplicingMacro for UnquoteSplicingMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Vec<Value>> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let mut arg: Value = args[0].clone();
        if let Value::Symbol(sym) = arg {
            arg = environment.borrow().get(&sym)?;
        }

        let mut result: Vec<Value> = vec![];

        match arg {
            Value::List(x) => {
                for x in x.value.iter() {
                    result.push(eval(x.clone(), environment.clone())?);
                }
            }
            Value::Vector(x) => {
                for x in x.value.iter() {
                    result.push(eval(x.clone(), environment.clone())?);
                }
            }
            Value::Set(x) => {
                for x in x.value.iter() {
                    result.push(eval(x.clone(), environment.clone())?);
                }
            }
            Value::Map(m) => {
                for (k, v) in m.value.iter() {
                    let x = Value::Vector(Vector::from([k.clone(), v.clone()].to_vec()));
                    result.push(eval(x.clone(), environment.clone())?);
                }
            }
            Value::String(s) => {
                for c in s.chars() {
                    result.push(Value::String(c.to_string()));
                }
            }
            Value::Generator(g) => loop {
                let next = g.borrow_mut().next();
                match next {
                    Some(v) => {
                        result.push(v);
                    }
                    None => break,
                }
            },
            _ => Err(Error::Type(
                "unquote-splicing: argument must be a list, vector, set, map, string, or generator"
                    .to_string(),
            ))?,
        }

        Ok(result)
    }
}

// TODO: expand(@)

impl fmt::Display for UnquoteSplicingMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: unquote-splicing(~@)>")
    }
}

// do
pub static SYMBOL_DO: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("do"),
    meta: Meta {
        doc: Cow::Borrowed("Evaluate a series of expressions and return the last result."),
        mutable: false,
    },
    hash: fxhash::hash("do"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMacro;

impl Macro for DoMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        let mut result = Value::Nil;
        for arg in args {
            result = eval(arg, environment.clone())?;
        }

        Ok(result)
    }
}

impl fmt::Display for DoMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: do>")
    }
}

// if
pub static SYMBOL_IF: Lazy<Symbol> = Lazy::new(|| {
    Symbol {
    name: Cow::Borrowed("if"),
    meta: Meta {
        doc: Cow::Borrowed("If the first argument is true, evaluate the second argument. Otherwise, evaluate the third argument."),
        mutable: false,
    },
    hash: fxhash::hash("if")
}
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfMacro;

impl Macro for IfMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 2 || args.len() > 3 {
            return Err(arity_error_range(2, 3, args.len()));
        }

        let condition = &args[0];
        let truthy = eval(condition.clone(), environment.clone())?;

        let result = if truthy.is_truthy() {
            let true_branch = &args[1];
            eval(true_branch.clone(), environment.clone())?
        } else {
            let false_branch = if args.len() == 3 {
                &args[2]
            } else {
                &Value::Nil
            };
            eval(false_branch.clone(), environment.clone())?
        };
        Ok(result)
    }
}

impl fmt::Display for IfMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: if>")
    }
}

// break
pub static SYMBOL_BREAK: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("break"),
    meta: Meta {
        doc: Cow::Borrowed("Break out of a while/for loop."),
        mutable: false,
    },
    hash: fxhash::hash("break"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreakMacro;

impl ControlFlowMacro for BreakMacro {
    fn call(&self, args: Vec<Value>) -> Result<ControlFlow<Value, Value>> {
        if args.len() > 2 {
            return Err(arity_error_range(0, 1, args.len()));
        }

        let mut result = Value::Nil;
        if args.len() == 1 {
            result = args[0].clone();
        }
        Ok(ControlFlow::Break(result))
    }
}

impl fmt::Display for BreakMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: break>")
    }
}

// continue
pub static SYMBOL_CONTINUE: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("continue"),
    meta: Meta {
        doc: Cow::Borrowed("Continue a while/for loop."),
        mutable: false,
    },
    hash: fxhash::hash("continue"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinueMacro;

impl ControlFlowMacro for ContinueMacro {
    fn call(&self, args: Vec<Value>) -> Result<ControlFlow<Value, Value>> {
        if !args.is_empty() {
            return Err(arity_error(0, args.len()));
        }

        Ok(ControlFlow::Continue(Value::Nil))
    }
}

impl fmt::Display for ContinueMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: continue>")
    }
}

// while
pub static SYMBOL_WHILE: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("while"),
    meta: Meta {
        doc: Cow::Borrowed("While the first expression is true, evaluate the second expression."),
        mutable: false,
    },
    hash: fxhash::hash("while"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileMacro;

impl Macro for WhileMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }

        let local_env = Environment::new_local_environment(environment.clone());
        local_env
            .borrow_mut()
            .insert(&SYMBOL_BREAK, Value::ControlFlowMacro(Rc::new(BreakMacro)))?;
        local_env.borrow_mut().insert(
            &SYMBOL_CONTINUE,
            Value::ControlFlowMacro(Rc::new(ContinueMacro)),
        )?;

        let condition = &args[0];
        let bodies = &args[1..];

        let mut ret = Value::Nil;
        let result = 'looptop: loop {
            let truthy = eval(condition.clone(), local_env.clone())?;

            if !truthy.is_truthy() {
                break ret;
            }

            for body in bodies {
                ret = eval(body.clone(), local_env.clone())?;
                if let Value::ControlFlow(c) = ret.clone() {
                    match c.as_ref() {
                        ControlFlow::Break(v) => break 'looptop v.clone(),
                        ControlFlow::Continue(_) => continue 'looptop,
                    }
                }
            }
        };

        Ok(result)
    }
}

impl fmt::Display for WhileMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: while>")
    }
}

// switch
pub static SYMBOL_SWITCH: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("switch"),
    meta: Meta {
        doc: Cow::Borrowed("Switch macro."),
        mutable: false,
    },
    hash: fxhash::hash("switch"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchMacro;

impl Macro for SwitchMacro {
    fn call(&self, args: Vec<Value>, _environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }

        if args[1..].len() % 2 != 0 {
            return Err(Error::Syntax(
                "switch: case and expression must be in pairs".to_string(),
            ));
        }

        let val = args[0].clone();
        let mut result = Value::Nil;

        for chunk in args[1..].chunks(2) {
            let case = &chunk[0];
            let expr = &chunk[1];

            match case {
                Value::Vector(case) => {
                    if case.value.iter().any(|v| *v == val) {
                        result = expr.clone();
                        break;
                    }
                }
                Value::Keyword(case) => {
                    if case.name == ":default" {
                        result = expr.clone();
                        break;
                    } else {
                        return Err(Error::Syntax(
                            "switch: case must be a vector or :default keyword".to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(Error::Syntax(
                        "switch: case must be a vector or :default keyword".to_string(),
                    ))
                }
            }
        }
        Ok(result)
    }
}

impl fmt::Display for SwitchMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: switch>")
    }
}

// time
pub static SYMBOL_TIME: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("time"),
    meta: Meta {
        doc: Cow::Borrowed("Time the evaluation of an expression."),
        mutable: false,
    },
    hash: fxhash::hash("time"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeMacro;

impl Macro for TimeMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let start = std::time::Instant::now();
        let result = eval(args[0].clone(), environment)?;
        let end = std::time::Instant::now();
        println!("Elapsed time: {:?}", end - start);

        Ok(result)
    }
}

impl fmt::Display for TimeMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: time>")
    }
}

// doc
pub static SYMBOL_DOC: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("doc"),
    meta: Meta {
        doc: Cow::Borrowed("Get the documentation of a value."),
        mutable: false,
    },
    hash: fxhash::hash("doc"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocMacro;

impl Macro for DocMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let sym = match &args[0] {
            Value::Symbol(sym) => sym,
            _ => {
                return Err(Error::Type(
                    "doc: first argument must be a symbol".to_string(),
                ))
            }
        };

        let (key, val) = environment.borrow().get_key_value(sym)?;

        let mut result = "------------------------------\n".to_string();
        result += format!("{}: {}\n", val.type_name(), key.name).as_str();
        result += format!("{}\n", key.meta.doc).as_str();
        result += "------------------------------";

        // TODO: generate doc about arity

        println!("{}", result);

        Ok(Value::Nil)
    }
}

impl fmt::Display for DocMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: doc>")
    }
}

// fn
pub static SYMBOL_FN: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("fn"),
    meta: Meta {
        doc: Cow::Borrowed("Create a anonymous/lambda function."),
        mutable: false,
    },
    hash: fxhash::hash("fn"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnMacro;

impl Macro for FnMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 2 {
            return Err(arity_error_min(2, args.len()));
        }

        let mut params = vec![];
        let mut body = vec![];

        for (i, arg) in args.into_iter().enumerate() {
            if i == 0 {
                let params_vec = match arg {
                    Value::Vector(v) => v.value,
                    _ => {
                        return Err(Error::Type(
                            "fn: first argument must be a vector".to_string(),
                        ))
                    }
                };

                for param in params_vec {
                    match param {
                        Value::Symbol(sym) => params.push(sym),
                        _ => {
                            return Err(Error::Type(
                                "fn: first argument must be a vector of symbols".to_string(),
                            ))
                        }
                    }
                }
            } else {
                body.push(arg);
            }
        }

        Ok(Value::Function(Rc::new(Lambda {
            args: params,
            body,
            environment: environment.clone(),
        })))
    }
}

impl fmt::Display for FnMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: fn>")
    }
}

// defn
pub static SYMBOL_DEFN: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("defn"),
    meta: Meta {
        doc: Cow::Borrowed("Bind a function to a symbol."),
        mutable: false,
    },
    hash: fxhash::hash("defn"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefnMacro;

impl Macro for DefnMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 2 {
            return Err(arity_error_min(2, args.len()));
        }

        let mut symbol = match args[0].clone() {
            Value::Symbol(sym) => sym,
            _ => {
                return Err(Error::Type(
                    "defn: first argument must be a symbol".to_string(),
                ))
            }
        };

        let params;
        let bodies: &[Value];

        if args.len() == 2 {
            if let Value::Vector(_) = args[1] {
                params = args[1].clone();
                bodies = &[Value::Nil];
            } else {
                return Err(Error::Syntax("defn: informal form".to_string()));
            }
        } else {
            match &args[1] {
                Value::String(s) => {
                    if let Value::Vector(_) = args[2] {
                        symbol.meta.doc = Cow::Owned(s.clone());
                        params = args[2].clone();
                        bodies = &[Value::Nil];
                    } else {
                        return Err(Error::Syntax("defn: informal form".to_string()));
                    }
                }
                Value::Vector(_) => {
                    params = args[1].clone();
                    bodies = &args[2..];
                }
                _ => return Err(Error::Syntax("defn: informal form".to_string())),
            }
        }

        let mut symbols = vec![];
        for p in params {
            match p {
                Value::Symbol(sym) => symbols.push(sym),
                _ => {
                    return Err(Error::Syntax(
                        "defn: parameters must be symbols".to_string(),
                    ))
                }
            }
        }

        let exec_bodies = bodies.to_vec();

        let lambda = Lambda {
            args: symbols,
            body: exec_bodies,
            environment: environment.clone(),
        };

        environment
            .borrow_mut()
            .insert(&symbol, Value::Function(Rc::new(lambda)))?;

        Ok(Value::Symbol(symbol.clone()))
    }
}

impl fmt::Display for DefnMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: defn>")
    }
}

// thread-first(->)
pub static SYMBOL_THREAD_FIRST: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("->"),
    meta: Meta {
        doc: Cow::Borrowed("Thread-first macro."),
        mutable: false,
    },
    hash: fxhash::hash("->"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadFirstMacro;

impl Macro for ThreadFirstMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }

        let mut result = eval(args[0].clone(), environment.clone())?;

        for arg in args.into_iter().skip(1) {
            match arg {
                Value::List(mut list) => {
                    list.value.insert(1, result.clone());
                    result = eval(Value::List(list), environment.clone())?;
                }
                Value::Symbol(sym) => {
                    let new_list = Value::as_list(vec![Value::Symbol(sym), result.clone()])?;
                    result = eval(new_list, environment.clone())?;
                }
                _ => {
                    return Err(Error::Type(
                        "->: arguments must be lists, functions or macros".to_string(),
                    ))
                }
            }
        }

        Ok(result)
    }
}

impl fmt::Display for ThreadFirstMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: thread-first(->)>")
    }
}

// thread-last(->>)
pub static SYMBOL_THREAD_LAST: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("->>"),
    meta: Meta {
        doc: Cow::Borrowed("Thread-last macro."),
        mutable: false,
    },
    hash: fxhash::hash("->>"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadLastMacro;

impl Macro for ThreadLastMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }

        let mut result = eval(args[0].clone(), environment.clone())?;

        for arg in args.into_iter().skip(1) {
            match arg {
                Value::List(mut list) => {
                    list.value.push(result);
                    result = eval(Value::List(list), environment.clone())?;
                }
                Value::Symbol(sym) => {
                    let new_list = Value::as_list(vec![Value::Symbol(sym), result.clone()])?;
                    result = eval(new_list, environment.clone())?;
                }
                _ => {
                    return Err(Error::Type(
                        "->: arguments must be lists, functions or macros".to_string(),
                    ))
                }
            }
        }

        Ok(result)
    }
}

impl fmt::Display for ThreadLastMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: thread-last(->>)>")
    }
}

// cond
pub static SYMBOL_COND: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("cond"),
    meta: Meta {
        doc: Cow::Borrowed("Cond macro."),
        mutable: false,
    },
    hash: fxhash::hash("cond"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CondMacro;

impl Macro for CondMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() % 2 != 0 {
            return Err(Error::Syntax(
                "cond: case and expression must be in pairs".to_string(),
            ));
        }

        let mut result = Value::Nil;
        let keyword_else = Value::Keyword(Keyword {
            name: ":else".to_string(),
        });

        for chunk in args.chunks(2) {
            let case = eval(chunk[0].clone(), environment.clone())?;

            if case.is_truthy() || case == keyword_else {
                result = eval(chunk[1].clone(), environment)?;
                break;
            }
        }
        Ok(result)
    }
}

impl fmt::Display for CondMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: cond>")
    }
}

// and
pub static SYMBOL_AND: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("and"),
    meta: Meta {
        doc: Cow::Borrowed("And macro."),
        mutable: false,
    },
    hash: fxhash::hash("and"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndMacro;

impl Macro for AndMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Ok(Value::Bool(true));
        }
        let mut result = Value::Bool(true);
        for arg in args {
            result = eval(arg, environment.clone())?;
            if !result.is_truthy() {
                break;
            }
        }
        Ok(result)
    }
}

impl fmt::Display for AndMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: and>")
    }
}

// or
pub static SYMBOL_OR: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("or"),
    meta: Meta {
        doc: Cow::Borrowed("Or macro."),
        mutable: false,
    },
    hash: fxhash::hash("or"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrMacro;

impl Macro for OrMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Ok(Value::Nil);
        }
        let mut result = Value::Bool(false);
        for arg in args {
            result = eval(arg, environment.clone())?;
            if result.is_truthy() {
                break;
            }
        }
        Ok(result)
    }
}

impl fmt::Display for OrMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: or>")
    }
}

// for
pub static SYMBOL_FOR: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("for"),
    meta: Meta {
        doc: Cow::Borrowed("For macro."),
        mutable: false,
    },
    hash: fxhash::hash("for"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForMacro;

impl Macro for ForMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 2 {
            return Err(arity_error_min(2, args.len()));
        }

        let local_env = Environment::new_local_environment(environment.clone());
        local_env
            .borrow_mut()
            .insert(&SYMBOL_BREAK, Value::ControlFlowMacro(Rc::new(BreakMacro)))?;
        local_env.borrow_mut().insert(
            &SYMBOL_CONTINUE,
            Value::ControlFlowMacro(Rc::new(ContinueMacro)),
        )?;

        let binding = match args[0].clone() {
            Value::Vector(v) => v,
            _ => Err(Error::Type(
                "for: first argument must be a vector".to_string(),
            ))?,
        };

        if binding.value.len() != 2 {
            Err(Error::Type(
                "for: first argument must be a vector of length 2".to_string(),
            ))?
        }

        let param_symbol = match binding.value[0].clone() {
            Value::Symbol(sym) => sym,
            _ => Err(Error::Type(
                "for: first element of binding must be a symbol".to_string(),
            ))?,
        };

        let param_body = binding.value[1].clone();

        let mut iterator = match param_body {
            Value::Symbol(_)
            | Value::List(_)
            | Value::Vector(_)
            | Value::Set(_)
            | Value::Map(_) => eval(param_body, local_env.clone())?,
            Value::Generator(g) => Value::Generator(g),
            _ => Err(Error::Type(
                "for: second element of binding must be a symbol, list, vector, set, or map"
                    .to_string(),
            ))?,
        }
        .into_iter();

        // TODO: slow down
        local_env.borrow_mut().insert(&param_symbol, Value::Nil)?;

        let mut result = Value::Nil;
        'looptop: loop {
            let v = iterator.next();
            if v.is_none() {
                break;
            }

            // TODO: slow down
            local_env.borrow_mut().set(&param_symbol, v.unwrap())?;

            for arg in args.iter().skip(1) {
                let ret = eval(arg.clone(), local_env.clone())?;
                if let Value::ControlFlow(c) = ret {
                    match c.as_ref() {
                        ControlFlow::Break(v) => {
                            result = v.clone();
                            break 'looptop;
                        }
                        ControlFlow::Continue(_) => continue 'looptop,
                    }
                } else {
                    result = ret;
                }
            }
        }
        Ok(result)
    }
}

impl fmt::Display for ForMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: for>")
    }
}

// gensym
pub static SYMBOL_GENSYM: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("gensym"),
    meta: Meta {
        doc: Cow::Borrowed("Create a unique symbol."),
        mutable: false,
    },
    hash: fxhash::hash("gensym"),
});

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GensymMacro;

impl Macro for GensymMacro {
    fn call(&self, args: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() > 1 {
            return Err(arity_error_range(0, 1, args.len()));
        }

        let name = if args.is_empty() {
            format!("gensym-{}", environment.borrow().gensym_id)
        } else {
            match args[0] {
                Value::String(ref s) => format!("{}-{}", s, environment.borrow().gensym_id),
                _ => return Err(type_error("string", args[0].type_name().as_str())),
            }
        };

        environment.borrow_mut().gensym_id += 1;

        Ok(Value::Symbol(Symbol {
            name: Cow::Owned(name.clone()),
            meta: Meta {
                doc: Cow::Borrowed("Generated symbol by gensym."),
                mutable: false,
            },
            hash: fxhash::hash(&name),
        }))
    }
}

impl fmt::Display for GensymMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: gensym>")
    }
}

// macro
pub static SYMBOL_MACRO: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("macro"),
    meta: Meta {
        doc: Cow::Borrowed("Create a macro."),
        mutable: false,
    },
    hash: fxhash::hash("macro"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroMacro;

impl Macro for MacroMacro {
    fn call(&self, args: Vec<Value>, _environment: Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 2 {
            return Err(arity_error_min(2, args.len()));
        }
        Ok(Value::Nil)
    }
}

impl fmt::Display for MacroMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: macro>")
    }
}

// TODO:
// MacroMacro,
// /AUTO-GENSYM
// EnumMacro,
// StructMacro,
// ClassMacro,
// NameSpaceMacro
// ReturnMacro
