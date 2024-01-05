/* core/builtin/macros.rs */

use std::borrow::Cow;
use std::vec;

use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::error::{arity_error, arity_error_min, arity_error_range};
#[allow(unused_imports)]
use crate::core::types::function::Function;
use crate::core::types::meta::Meta;
use crate::core::types::r#macro::Macro;
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
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }

        let mut args_for_def: Vec<Value> = vec![];
        for (i, v) in args.into_iter().enumerate() {
            if i == 0 {
                args_for_def.push(v);
            } else {
                ast.push(v.clone()); // NOTE: need to clone here
                args_for_def.push(evalfn(environment, ast)?);
            }
        }

        let symbol = match args_for_def[0].clone() {
            Value::Symbol(sym) => sym,
            _ => {
                return Err(Error::Type(
                    "def: first argument must be a symbol".to_string(),
                ))
            }
        };

        let value = args_for_def[1].clone();

        environment.put(&symbol, value)?;
        Ok(Value::Symbol(symbol))
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
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }

        let mut args_for_def: Vec<Value> = vec![];
        for (i, v) in args.into_iter().enumerate() {
            if i == 0 {
                args_for_def.push(v);
            } else {
                ast.push(v.clone()); // NOTE: need to clone here
                args_for_def.push(evalfn(environment, ast)?);
            }
        }

        let mut symbol = match args_for_def[0].clone() {
            Value::Symbol(sym) => sym,
            _ => {
                return Err(Error::Type(
                    "const: first argument must be a symbol".to_string(),
                ))
            }
        };

        let value = args_for_def[1].clone();
        symbol.meta.mutable = false;

        environment.put(&symbol, value)?;
        Ok(Value::Symbol(symbol))
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
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }

        let mut args_for_set: Vec<Value> = vec![];
        for (i, v) in args.into_iter().enumerate() {
            if i == 0 {
                args_for_set.push(v);
            } else {
                ast.push(v.clone()); // NOTE: need to clone here
                args_for_set.push(evalfn(environment, ast)?);
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

        environment.get(&symbol)?;
        environment.put(&symbol, value)?;
        Ok(Value::Symbol(symbol))
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
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() < 1 {
            return Err(arity_error_min(1, args.len()));
        }

        let mut local_env = Environment::new(None, Some(environment));

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

        bind_form.value.chunks(2).for_each(|chunk| {
            let symbol = match &chunk[0] {
                Value::Symbol(sym) => Ok(sym),
                _ => Err(Error::Type(
                    "let: first element of each pair must be a symbol".to_string(),
                )),
            };

            let value = chunk[1].clone();

            local_env.put(symbol.unwrap(), value).unwrap();
        });

        let mut result = Value::Nil;
        for arg in args.into_iter().skip(1) {
            ast.push(arg.clone());
            result = evalfn(&mut local_env, ast)?;
        }

        Ok(result)
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
    fn call(
        &self,
        args: Vec<Value>,
        _environment: &mut Environment,
        _ast: &mut Vec<Value>,
        _evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Ok(args[0].clone())
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
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        // TODO:
        let mut local_env = Environment::new(None, Some(environment));
        // local_env.put(&UNQUOTE.name, Value::Macro(UNQUOTE))?;
        // local_env.put(&UNQUOTE_SPLICING.name, Value::Macro(UNQUOTE_SPLICING))?;
        // local_env.put(&SYMBOL_SYNTAX_QUOTING, Value::Bool(true))?;

        ast.push(args[0].clone());
        evalfn(&mut local_env, ast)
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
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let mut local_env = Environment::new(None, Some(environment));
        local_env.put(&SYMBOL_UNQUOTING, Value::Bool(true))?;

        ast.push(args[0].clone());
        evalfn(&mut local_env, ast)
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

impl Macro for UnquoteSplicingMacro {
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let mut local_env = Environment::new(None, Some(environment));
        local_env.put(&SYMBOL_UNQUOTING, Value::Bool(true))?;

        let mut arg: Value = args[0].clone();
        if let Value::Symbol(sym) = arg {
            arg = environment.get(&sym)?.1.clone();
        }

        let mut result: Vec<Value> = vec![];

        match arg {
            Value::List(l) => {
                for v in l.value.iter() {
                    ast.push(v.clone());
                    result.push(evalfn(&mut local_env, ast)?);
                }
            }
            Value::Vector(v) => {
                for v in v.value.iter() {
                    ast.push(v.clone());
                    result.push(evalfn(&mut local_env, ast)?);
                }
            }
            Value::Set(s) => {
                for v in s.value.iter() {
                    ast.push(v.clone());
                    result.push(evalfn(&mut local_env, ast)?);
                }
            }
            Value::Map(m) => {
                for (k, v) in m.value.iter() {
                    ast.push(Value::Vector(Vector::from([k.clone(), v.clone()].to_vec())));
                    result.push(evalfn(&mut local_env, ast)?);
                }
            }
            Value::String(s) => {
                for c in s.chars() {
                    result.push(Value::String(c.to_string()));
                }
            }
            _ => Err(Error::Type(
                "unquote-splicing: argument must be a list, vector, set, map, or string"
                    .to_string(),
            ))?,
        }

        if let Some(parent) = ast.last() {
            match parent {
                Value::List(l) => {
                    if let Value::Symbol(sym) = l.value[0].clone() {
                        if sym == SYMBOL_UNQUOTE_SPLICING {
                            Err(Error::Syntax(
                                "unquote-splicing: parent must be a list, vector or set"
                                    .to_string(),
                            ))?
                        }
                    }
                }
                Value::Vector(_) => {}
                Value::Set(_) => {}
                _ => Err(Error::Syntax(
                    "unquote-splicing: parent must be a list, vector, or set".to_string(),
                ))?,
            }
        }

        Value::as_list(result)
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
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        let mut result = Value::Nil;
        for arg in args {
            ast.push(arg.clone());
            result = evalfn(environment, ast)?;
        }

        Ok(result)
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
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() < 2 || args.len() > 3 {
            return Err(arity_error_range(2, 3, args.len()));
        }

        let condition = &args[0];
        ast.push(condition.clone());
        let truthy = evalfn(environment, ast)?;

        if truthy.is_truthy() {
            let true_branch = &args[1];
            ast.push(true_branch.clone());
        } else {
            let false_branch = if args.len() == 3 {
                &args[2]
            } else {
                &Value::Nil
            };
            ast.push(false_branch.clone());
        }
        let result = evalfn(environment, ast)?;
        Ok(result)
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
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        ast: &mut Vec<Value>,
        evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }

        let condition = &args[0];
        let body = &args[1];

        let mut ret = Value::Nil;
        let result = loop {
            ast.push(condition.clone());
            let truthy = evalfn(environment, ast)?;

            if truthy.is_truthy() {
                ast.push(body.clone());
                ret = evalfn(environment, ast)?;
            } else {
                break ret;
            }
        };

        Ok(result)
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
    fn call(
        &self,
        args: Vec<Value>,
        _environment: &mut Environment,
        _ast: &mut Vec<Value>,
        _evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() < 1 {
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
    fn call(
        &self,
        args: Vec<Value>,
        _environment: &mut Environment,
        _ast: &mut Vec<Value>,
        _evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let start = std::time::Instant::now();
        let result = args[0].clone();
        let end = std::time::Instant::now();
        println!("Elapsed time: {:?}", end - start);

        Ok(result)
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
    fn call(
        &self,
        args: Vec<Value>,
        environment: &mut Environment,
        _ast: &mut Vec<Value>,
        _evalfn: fn(&mut Environment, &mut Vec<Value>) -> Result<Value>,
    ) -> Result<Value> {
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

        let (key, val) = environment.get(sym)?;

        let mut result = "------------------------------\n".to_string();
        result += format!("{}: {}\n", val.type_name(), sym.name).as_str();
        result += format!("{}\n", key.meta.doc).as_str();
        result += "------------------------------";

        // TODO: generate doc about arity

        println!("{}", result);

        Ok(Value::Nil)
    }
}

// fn
// TODO:
// SliceMacro,[-1:3, :3]
// FnMacro,
// DefnMacro,
// ForMacro,
// BreakMacro,
// ContinueMacro,
// EnumMacro,
// StructMacro,
// MacroMacro,
// ClassMacro,
// ThreadArrowMacro,
// DoubleThreadArrowMacro
// AND, OR
// GENSYM/AUTO-GENSYM
// SEQUENCE/RANGE

pub const SYMBOL_SYNTAX_QUOTING: Symbol = Symbol {
    name: Cow::Borrowed("*syntax-quoting*"),
    meta: Meta {
        doc: Cow::Borrowed("Internal variable for syntax-quoting."),
        mutable: false,
    },
};

pub const SYMBOL_UNQUOTING: Symbol = Symbol {
    name: Cow::Borrowed("*unquoting*"),
    meta: Meta {
        doc: Cow::Borrowed("Internal variable for unquoting."),
        mutable: false,
    },
};
