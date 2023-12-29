/* core/builtin/macros.rs */

use std::borrow::Cow;
use std::vec;

use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::error::{arity_error, arity_error_min, arity_error_range};
use crate::core::types::meta::Meta;
use crate::core::types::r#macro::Macro;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

// ===== Reader macros
pub const SYMBOL_QUOTE: Symbol = Symbol {
    name: Cow::Borrowed("quote"),
    meta: Meta {
        doc: Cow::Borrowed("Quote a value."),
        mutable: false,
    },
};

pub const SYMBOL_SYNTAX_QUOTE: Symbol = Symbol {
    name: Cow::Borrowed("syntax-quote"),
    meta: Meta {
        doc: Cow::Borrowed("Syntax-quote a value."),
        mutable: false,
    },
};

pub const SYMBOL_UNQUOTE: Symbol = Symbol {
    name: Cow::Borrowed("unquote"),
    meta: Meta {
        doc: Cow::Borrowed("Unquote a value."),
        mutable: false,
    },
};

pub const SYMBOL_UNQUOTE_SPLICING: Symbol = Symbol {
    name: Cow::Borrowed("unquote-splicing"),
    meta: Meta {
        doc: Cow::Borrowed("Unquote-splicing a value."),
        mutable: false,
    },
};

// ===== Core macros
pub const DEF: Macro = Macro {
    name: Symbol {
        name: Cow::Borrowed("def"),
        meta: Meta {
            doc: Cow::Borrowed("Bind a value to a symbol."),
            mutable: false,
        },
    },
    func: |args, environment, ast, evalfn| {
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
    },
};

pub const QUOTE: Macro = Macro {
    name: SYMBOL_QUOTE,
    func: |args, _, _, _| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Ok(args[0].clone())
    },
};

pub const UNQUOTE: Macro = Macro {
    name: SYMBOL_UNQUOTE,
    func: |args, environment, ast, evalfn| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let arg = &args[0];

        let val: Result<Value> = match arg {
            Value::Symbol(symbol) => {
                let value = environment.get(&symbol)?;
                Ok(value.clone())
            }
            _ => Ok(arg.clone()),
        };

        ast.push(val?);
        evalfn(environment, ast)
    },
};

pub const UNQUOTE_SPLICING: Macro = Macro {
    name: SYMBOL_UNQUOTE_SPLICING,
    func: |args, environment, ast, evalfn| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let arg = &args[0];

        let value = match arg {
            Value::Symbol(symbol) => environment.get(&symbol)?.clone(),
            _ => arg.clone(),
        };

        // if value is collectionn then splice
        match value {
            Value::List(list) => list.value.into_iter().for_each(|v| ast.push(v)),
            Value::Vector(vector) => vector.value.into_iter().for_each(|v| ast.push(v)),
            Value::Map(map) => map.value.into_iter().for_each(|(k, v)| {
                ast.push(Value::as_vector([k, v].to_vec()).unwrap());
            }),
            Value::Set(set) => set.value.into_iter().for_each(|v| ast.push(v)),
            _ => ast.push(value),
        };

        Ok(Value::Nil)
    },
};

pub const SYNTAX_QUOTE: Macro = Macro {
    name: SYMBOL_SYNTAX_QUOTE,
    func: |args, environment, ast, evalfn| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let mut local_env = Environment::new(None, Some(environment));
        local_env.put(&UNQUOTE.name, Value::Macro(UNQUOTE))?;
        local_env.put(&UNQUOTE_SPLICING.name, Value::Macro(UNQUOTE_SPLICING))?;

        let value = args[0].clone();
        let quoted = Value::as_list(vec![Value::Symbol(SYMBOL_QUOTE), value])?;

        ast.push(quoted);
        let result = evalfn(&mut local_env, ast)?;

        Ok(result)
    },
};

pub const TIME: Macro = Macro {
    name: Symbol {
        name: Cow::Borrowed("time"),
        meta: Meta {
            doc: Cow::Borrowed("Time the evaluation of an expression."),
            mutable: false,
        },
    },
    func: |args, environment, ast, evalfn| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let start = std::time::Instant::now();
        ast.push(args[0].clone());
        let result = evalfn(environment, ast)?;

        let end = std::time::Instant::now();
        println!("Elapsed time: {:?}", end - start);

        Ok(result)
    },
};

pub const DO: Macro = Macro {
    name: Symbol {
        name: Cow::Borrowed("do"),
        meta: Meta {
            doc: Cow::Borrowed("Evaluate a series of expressions and return the last result."),
            mutable: false,
        },
    },
    func: |args, environment, ast, evalfn| {
        let mut result = Value::Nil;
        for arg in args {
            ast.push(arg.clone());
            result = evalfn(environment, ast)?;
        }

        Ok(result)
    },
};

pub const CONST: Macro = Macro {
    name: Symbol {
        name: Cow::Borrowed("const"),
        meta: Meta {
            doc: Cow::Borrowed("Bind a value to a symbol."),
            mutable: false,
        },
    },
    func: |args, environment, ast, evalfn| {
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
    },
};

pub const SET: Macro = Macro {
    name: Symbol {
        name: Cow::Borrowed("set!"),
        meta: Meta {
            doc: Cow::Borrowed("Bind a value to a symbol."),
            mutable: false,
        },
    },
    func: |args, environment, ast, evalfn| {
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
    },
};

pub const IF: Macro = Macro {
    name: Symbol {
        name: Cow::Borrowed("if"),
        meta: Meta {
            doc: Cow::Borrowed("If the first argument is true, evaluate the second argument. Otherwise, evaluate the third argument."),
            mutable: false,
        },
    },
    func: |args, environment, ast, evalfn| {
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
    },
};

pub const WHILE: Macro = Macro {
    name: Symbol {
        name: Cow::Borrowed("while"),
        meta: Meta {
            doc: Cow::Borrowed(
                "While the first expression is true, evaluate the second expression.",
            ),
            mutable: false,
        },
    },
    func: |args, environment, ast, evalfn| {
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
    },
};

pub const LET: Macro = Macro {
    name: Symbol {
        name: Cow::Borrowed("let"),
        meta: Meta {
            doc: Cow::Borrowed("Bind a value to a symbol in a local scope."),
            mutable: false,
        },
    },
    func: |args, environment, ast, evalfn| {
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
                Value::Symbol(sym) => sym,
                _ => todo!(),
            };

            let value = chunk[1].clone();

            local_env.put(symbol, value).unwrap();
        });

        let mut result = Value::Nil;
        for arg in args.into_iter().skip(1) {
            ast.push(arg.clone());
            result = evalfn(&mut local_env, ast)?;
        }

        Ok(result)
    },
};

// TODO: performance?
pub const SWITCH: Macro = Macro {
    name: Symbol {
        name: Cow::Borrowed("switch"),
        meta: Meta {
            doc: Cow::Borrowed("Switch macro."),
            mutable: false,
        },
    },
    func: |args, environment, ast, evalfn| {
        if args.len() < 1 {
            return Err(arity_error_min(1, args.len()));
        }

        if args[1..].len() % 2 != 0 {
            return Err(Error::Syntax(
                "switch: case and expression must be in pairs".to_string(),
            ));
        }

        ast.push(args[0].clone());
        let val = evalfn(environment, ast)?;
        let mut result = Value::Nil;

        for chunk in args[1..].chunks(2) {
            let case = &chunk[0];
            let expr = &chunk[1];

            match case {
                Value::Vector(case) => {
                    if case.value.iter().any(|v| *v == val) {
                        ast.push(expr.clone());
                        result = evalfn(environment, ast)?;
                        break;
                    }
                }
                Value::Keyword(case) => {
                    if case.name == ":default" {
                        ast.push(expr.clone());
                        result = evalfn(environment, ast)?;
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
    },
};

pub const ALL_MACROS: [Value; 11] = [
    Value::Macro(DEF),
    Value::Macro(QUOTE),
    Value::Macro(SYNTAX_QUOTE),
    Value::Macro(TIME),
    Value::Macro(DO),
    Value::Macro(CONST),
    Value::Macro(SET),
    Value::Macro(IF),
    Value::Macro(WHILE),
    Value::Macro(LET),
    Value::Macro(SWITCH),
];

// TODO:
// SliceMacro,
// FnMacro,
// ForMacro,
// BreakMacro,
// ContinueMacro,
// EnumMacro,
// StructMacro,
// MacroMacro,
// ClassMacro,
// ThreadMacro,
