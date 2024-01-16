/* core/builtin/macros.rs */

use std::borrow::Cow;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::vec;

use crate::core::environment::Environment;
use crate::core::eval::eval;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::error::{arity_error, arity_error_min, arity_error_range};
use crate::core::types::keyword::Keyword;
use crate::core::types::lambda::Lambda;
use crate::core::types::meta::Meta;
use crate::core::types::r#macro::Macro;
use crate::core::types::r#macro::SplicingMacro;
use crate::core::types::symbol::Symbol;
use crate::core::types::vector::Vector;
use crate::core::value::Value;

// def
pub const SYMBOL_DEF: Symbol = Symbol {
    name: Cow::Borrowed("def"),
    meta: Meta {
        doc: Cow::Borrowed("Bind a value to a symbol."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefMacro;

impl Macro for DefMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 2 && args.len() > 3 {
            return Err(arity_error_range(2, 3, args.len()));
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

        if args.len() == 3 {
            let docstring = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::Type("def: docstring must be a string".to_string())),
            };

            symbol.meta.doc = Cow::Owned(docstring);
            body = args[2].clone();
        } else {
            body = args[1].clone();
        }

        let value = eval(body, environment)?;

        environment.borrow_mut().insert_to_root(&symbol, value)?;

        Ok(Value::Symbol(symbol))
    }
}

impl fmt::Display for DefMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: def>")
    }
}

// const
pub const SYMBOL_CONST: Symbol = Symbol {
    name: Cow::Borrowed("const"),
    meta: Meta {
        doc: Cow::Borrowed("Bind a value to a symbol."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstMacro;

impl Macro for ConstMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 2 && args.len() > 3 {
            return Err(arity_error_range(2, 3, args.len()));
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

        if args.len() == 3 {
            let docstring = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::Type("const: docstring must be a string".to_string())),
            };

            symbol.meta.doc = Cow::Owned(docstring);
            body = args[2].clone();
        } else {
            body = args[1].clone();
        }

        let value = eval(body, environment)?;

        symbol.meta.mutable = false;

        environment.borrow_mut().insert_to_root(&symbol, value)?;

        Ok(Value::Symbol(symbol))
    }
}

impl fmt::Display for ConstMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: const>")
    }
}

// set!
pub const SYMBOL_SET: Symbol = Symbol {
    name: Cow::Borrowed("set!"),
    meta: Meta {
        doc: Cow::Borrowed("Bind a value to a symbol."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetMacro;

impl Macro for SetMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }

        let mut args_for_set: Vec<Value> = vec![];
        for (i, v) in args.into_iter().enumerate() {
            if i == 0 {
                args_for_set.push(v);
            } else {
                args_for_set.push(eval(v, environment)?);
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
pub const SYMBOL_LET: Symbol = Symbol {
    name: Cow::Borrowed("let"),
    meta: Meta {
        doc: Cow::Borrowed("Bind a value to a symbol in a local scope."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetMacro;

impl Macro for LetMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }

        let local_env = Environment::new_local_environment(environment.clone());

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

            local_env.borrow_mut().insert_to_current(key?, val)?;
        }

        let mut result = Value::Nil;
        for arg in args.into_iter().skip(1) {
            result = eval(arg, environment)?
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
pub const SYMBOL_QUOTE: Symbol = Symbol {
    name: Cow::Borrowed("quote"),
    meta: Meta {
        doc: Cow::Borrowed("Quote a value."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuoteMacro;

impl Macro for QuoteMacro {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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
pub const SYMBOL_SYNTAX_QUOTE: Symbol = Symbol {
    name: Cow::Borrowed("syntax-quote"),
    meta: Meta {
        doc: Cow::Borrowed("Syntax-quote a value."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxQuoteMacro;

impl Macro for SyntaxQuoteMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let local_env = Environment::new_local_environment(environment.clone());
        local_env.borrow_mut().insert_to_current(
            &SYMBOL_UNQUOTE,
            Value::Macro(Rc::new(RefCell::new(UnquoteMacro))),
        )?;
        local_env.borrow_mut().insert_to_current(
            &SYMBOL_UNQUOTE_SPLICING,
            Value::SplicingMacro(Rc::new(RefCell::new(UnquoteSplicingMacro))),
        )?;

        eval(args[0].clone(), &local_env)
    }
}

impl fmt::Display for SyntaxQuoteMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: syntax-quote(`)>")
    }
}

// unquote(~)
pub const SYMBOL_UNQUOTE: Symbol = Symbol {
    name: Cow::Borrowed("unquote"),
    meta: Meta {
        doc: Cow::Borrowed("Unquote a value."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnquoteMacro;

impl Macro for UnquoteMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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
pub const SYMBOL_UNQUOTE_SPLICING: Symbol = Symbol {
    name: Cow::Borrowed("unquote-splicing"),
    meta: Meta {
        doc: Cow::Borrowed("Unquote-splicing a value."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnquoteSplicingMacro;

// NOTE: impl 'SplicingMacro'
impl SplicingMacro for UnquoteSplicingMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Vec<Value>> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let mut arg: Value = args[0].clone();
        if let Value::Symbol(sym) = arg {
            arg = environment.borrow().get(&sym)?.1.clone();
        }

        let mut result: Vec<Value> = vec![];

        match arg {
            Value::List(x) => {
                for x in x.value.iter() {
                    result.push(eval(x.clone(), environment)?);
                }
            }
            Value::Vector(x) => {
                for x in x.value.iter() {
                    result.push(eval(x.clone(), environment)?);
                }
            }
            Value::Set(x) => {
                for x in x.value.iter() {
                    result.push(eval(x.clone(), environment)?);
                }
            }
            Value::Map(m) => {
                for (k, v) in m.value.iter() {
                    let x = Value::Vector(Vector::from([k.clone(), v.clone()].to_vec()));
                    result.push(eval(x.clone(), environment)?);
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
pub const SYMBOL_DO: Symbol = Symbol {
    name: Cow::Borrowed("do"),
    meta: Meta {
        doc: Cow::Borrowed("Evaluate a series of expressions and return the last result."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMacro;

impl Macro for DoMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        let mut result = Value::Nil;
        for arg in args {
            result = eval(arg, environment)?;
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
pub const SYMBOL_IF: Symbol = Symbol {
    name: Cow::Borrowed("if"),
    meta: Meta {
        doc: Cow::Borrowed("If the first argument is true, evaluate the second argument. Otherwise, evaluate the third argument."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfMacro;

impl Macro for IfMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 2 || args.len() > 3 {
            return Err(arity_error_range(2, 3, args.len()));
        }

        let condition = &args[0];
        let truthy = eval(condition.clone(), environment)?;

        let result = if truthy.is_truthy() {
            let true_branch = &args[1];
            eval(true_branch.clone(), environment)?
        } else {
            let false_branch = if args.len() == 3 {
                &args[2]
            } else {
                &Value::Nil
            };
            eval(false_branch.clone(), environment)?
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
pub const SYMBOL_BREAK: Symbol = Symbol {
    name: Cow::Borrowed("break"),
    meta: Meta {
        doc: Cow::Borrowed("Break out of a while/for loop."),
        mutable: false,
    },
};

pub const SYMBOL_BREAKING: Symbol = Symbol {
    name: Cow::Borrowed("*breaking*"),
    meta: Meta {
        doc: Cow::Borrowed("Internal variable for break."),
        mutable: true,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreakMacro;

impl Macro for BreakMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() > 2 {
            return Err(arity_error_range(0, 1, args.len()));
        }

        environment
            .borrow_mut()
            .set(&SYMBOL_BREAKING, Value::Bool(true))?;

        let mut result = Value::Nil;
        if args.len() == 1 {
            result = args[0].clone();
        }
        Ok(result)
    }
}

impl fmt::Display for BreakMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: break>")
    }
}

// continue
pub const SYMBOL_CONTINUE: Symbol = Symbol {
    name: Cow::Borrowed("continue"),
    meta: Meta {
        doc: Cow::Borrowed("Continue a while/for loop."),
        mutable: false,
    },
};

pub const SYMBOL_CONTINUING: Symbol = Symbol {
    name: Cow::Borrowed("*continuing*"),
    meta: Meta {
        doc: Cow::Borrowed("Internal variable for continue."),
        mutable: true,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinueMacro;

impl Macro for ContinueMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if !args.is_empty() {
            return Err(arity_error(0, args.len()));
        }

        environment
            .borrow_mut()
            .set(&SYMBOL_CONTINUING, Value::Bool(true))?;

        Ok(Value::Nil)
    }
}

impl fmt::Display for ContinueMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: continue>")
    }
}

// while
pub const SYMBOL_WHILE: Symbol = Symbol {
    name: Cow::Borrowed("while"),
    meta: Meta {
        doc: Cow::Borrowed("While the first expression is true, evaluate the second expression."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileMacro;

impl Macro for WhileMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }

        let local_env = Environment::new_local_environment(environment.clone());
        local_env.borrow_mut().insert_to_current(
            &SYMBOL_BREAK,
            Value::Macro(Rc::new(RefCell::new(BreakMacro))),
        )?;
        local_env
            .borrow_mut()
            .insert_to_current(&SYMBOL_BREAKING, Value::Bool(false))?;
        local_env.borrow_mut().insert_to_current(
            &SYMBOL_CONTINUE,
            Value::Macro(Rc::new(RefCell::new(ContinueMacro))),
        )?;
        local_env
            .borrow_mut()
            .insert_to_current(&SYMBOL_CONTINUING, Value::Bool(false))?;

        let condition = &args[0];
        let bodies = &args[1..];

        let mut ret = Value::Nil;
        let result = 'looptop: loop {
            let truthy = eval(condition.clone(), &local_env)?;

            let prev_ret = ret.clone();
            if !truthy.is_truthy() {
                break ret;
            }
            for body in bodies {
                ret = eval(body.clone(), &local_env)?;
                if local_env.borrow().get(&SYMBOL_BREAKING)?.1.is_truthy() {
                    if ret == Value::Nil {
                        ret = prev_ret;
                    }
                    break 'looptop ret;
                }
                if local_env.borrow().get(&SYMBOL_CONTINUING)?.1.is_truthy() {
                    local_env
                        .borrow_mut()
                        .set(&SYMBOL_CONTINUING, Value::Bool(false))?;
                    continue 'looptop;
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
pub const SYMBOL_SWITCH: Symbol = Symbol {
    name: Cow::Borrowed("switch"),
    meta: Meta {
        doc: Cow::Borrowed("Switch macro."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchMacro;

impl Macro for SwitchMacro {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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
pub const SYMBOL_TIME: Symbol = Symbol {
    name: Cow::Borrowed("time"),
    meta: Meta {
        doc: Cow::Borrowed("Time the evaluation of an expression."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeMacro;

impl Macro for TimeMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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
pub const SYMBOL_DOC: Symbol = Symbol {
    name: Cow::Borrowed("doc"),
    meta: Meta {
        doc: Cow::Borrowed("Get the documentation of a value."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocMacro;

impl Macro for DocMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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

        let (key, val) = environment.borrow().get(sym)?;

        let mut result = "------------------------------\n".to_string();
        result += format!("{}: {}\n", val.type_name(), sym.name).as_str();
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
pub const SYMBOL_FN: Symbol = Symbol {
    name: Cow::Borrowed("fn"),
    meta: Meta {
        doc: Cow::Borrowed("Create a anonymous/lambda function."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnMacro;

impl Macro for FnMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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

        Ok(Value::Function(Rc::new(RefCell::new(Lambda {
            args: params,
            body,
            environment: environment.clone(),
        }))))
    }
}

impl fmt::Display for FnMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: fn>")
    }
}

// defn
pub const SYMBOL_DEFN: Symbol = Symbol {
    name: Cow::Borrowed("defn"),
    meta: Meta {
        doc: Cow::Borrowed("Bind a function to a symbol."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefnMacro;

impl Macro for DefnMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 3 || args.len() > 4 {
            return Err(arity_error_range(3, 4, args.len()));
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

        if args.len() == 4 {
            let docstring = match &args[1] {
                Value::String(s) => s.clone(),
                _ => return Err(Error::Type("defn: docstring must be a string".to_string())),
            };

            symbol.meta.doc = Cow::Owned(docstring);
            params = args[2].clone();
            bodies = &args[2..];
        } else {
            params = args[1].clone();
            bodies = &args[1..];
        }

        let params = match params {
            Value::Vector(v) => v.value,
            _ => {
                return Err(Error::Type(
                    "defn: parameters must be given via vector".to_string(),
                ))
            }
        };

        let mut symbols = vec![];
        for p in params {
            match p {
                Value::Symbol(sym) => symbols.push(sym),
                _ => return Err(Error::Type("defn: parameters must be symbols".to_string())),
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
            .insert_to_root(&symbol, Value::Function(Rc::new(RefCell::new(lambda))))?;

        Ok(Value::Symbol(symbol.clone()))
    }
}

impl fmt::Display for DefnMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: defn>")
    }
}

// thread-first(->)
pub const SYMBOL_THREAD_FIRST: Symbol = Symbol {
    name: Cow::Borrowed("->"),
    meta: Meta {
        doc: Cow::Borrowed("Thread-first macro."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadFirstMacro;

impl Macro for ThreadFirstMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }

        let mut result = eval(args[0].clone(), environment)?;

        for arg in args.into_iter().skip(1) {
            match arg {
                Value::List(mut list) => {
                    list.value.insert(1, result.clone());
                    result = eval(Value::List(list), environment)?;
                }
                Value::Symbol(sym) => {
                    let new_list = Value::as_list(vec![Value::Symbol(sym), result.clone()])?;
                    result = eval(new_list, environment)?;
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
pub const SYMBOL_THREAD_LAST: Symbol = Symbol {
    name: Cow::Borrowed("->>"),
    meta: Meta {
        doc: Cow::Borrowed("Thread-last macro."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadLastMacro;

impl Macro for ThreadLastMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }

        let mut result = eval(args[0].clone(), environment)?;

        for arg in args.into_iter().skip(1) {
            match arg {
                Value::List(mut list) => {
                    list.value.push(result);
                    result = eval(Value::List(list), environment)?;
                }
                Value::Symbol(sym) => {
                    let new_list = Value::as_list(vec![Value::Symbol(sym), result.clone()])?;
                    result = eval(new_list, environment)?;
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
pub const SYMBOL_COND: Symbol = Symbol {
    name: Cow::Borrowed("cond"),
    meta: Meta {
        doc: Cow::Borrowed("Cond macro."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CondMacro;

impl Macro for CondMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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
            let case = eval(chunk[0].clone(), environment)?;

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
pub const SYMBOL_AND: Symbol = Symbol {
    name: Cow::Borrowed("and"),
    meta: Meta {
        doc: Cow::Borrowed("And macro."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndMacro;

impl Macro for AndMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Ok(Value::Bool(true));
        }
        let mut result = Value::Bool(true);
        for arg in args {
            result = eval(arg, environment)?;
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
pub const SYMBOL_OR: Symbol = Symbol {
    name: Cow::Borrowed("or"),
    meta: Meta {
        doc: Cow::Borrowed("Or macro."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrMacro;

impl Macro for OrMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.is_empty() {
            return Ok(Value::Nil);
        }
        let mut result = Value::Bool(false);
        for arg in args {
            result = eval(arg, environment)?;
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
pub const SYMBOL_FOR: Symbol = Symbol {
    name: Cow::Borrowed("for"),
    meta: Meta {
        doc: Cow::Borrowed("For macro."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForMacro;

impl Macro for ForMacro {
    fn call(&self, args: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 2 {
            return Err(arity_error_min(2, args.len()));
        }

        let local_env = Environment::new_local_environment(environment.clone());
        local_env.borrow_mut().insert_to_current(
            &SYMBOL_BREAK,
            Value::Macro(Rc::new(RefCell::new(BreakMacro))),
        )?;
        local_env
            .borrow_mut()
            .insert_to_current(&SYMBOL_BREAKING, Value::Bool(false))?;
        local_env.borrow_mut().insert_to_current(
            &SYMBOL_CONTINUE,
            Value::Macro(Rc::new(RefCell::new(ContinueMacro))),
        )?;
        local_env
            .borrow_mut()
            .insert_to_current(&SYMBOL_CONTINUING, Value::Bool(false))?;

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
            | Value::Map(_) => eval(param_body, &local_env)?,
            Value::Generator(g) => Value::Generator(g),
            _ => Err(Error::Type(
                "for: second element of binding must be a symbol, list, vector, set, or map"
                    .to_string(),
            ))?,
        }
        .into_iter();

        local_env
            .borrow_mut()
            .insert_to_current(&param_symbol, Value::Nil)?;

        let mut ret = Value::Nil;
        let result = 'looptop: loop {
            let v = iterator.next();
            if v.is_none() {
                break ret;
            }

            local_env.borrow_mut().set(&param_symbol, v.unwrap())?;

            let prev_ret = ret.clone();
            for arg in args.iter().skip(1) {
                ret = eval(arg.clone(), &local_env)?;
                if local_env.borrow().get(&SYMBOL_BREAKING)?.1.is_truthy() {
                    if ret == Value::Nil {
                        ret = prev_ret;
                    }
                    break 'looptop ret;
                }
                if local_env.borrow().get(&SYMBOL_CONTINUING)?.1.is_truthy() {
                    local_env
                        .borrow_mut()
                        .set(&SYMBOL_CONTINUING, Value::Bool(false))?;
                    continue 'looptop;
                }
            }
        };
        Ok(result)
    }
}

impl fmt::Display for ForMacro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin macro: for>")
    }
}

// macro
pub const SYMBOL_MACRO: Symbol = Symbol {
    name: Cow::Borrowed("macro"),
    meta: Meta {
        doc: Cow::Borrowed("Create a macro."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroMacro;

impl Macro for MacroMacro {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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
