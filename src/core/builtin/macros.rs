/* core/builtin/macros.rs */

use std::borrow::Cow;

use crate::core::environment::Environment;
use crate::core::types::error::Error;
#[allow(unused_imports)]
use crate::core::types::error::{arity_error, arity_error_min, arity_error_range};
use crate::core::types::meta::Meta;
use crate::core::types::r#macro::Macro;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

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
    name: Symbol {
        name: Cow::Borrowed("quote"),
        meta: Meta {
            doc: Cow::Borrowed("Quote a value."),
            mutable: false,
        },
    },
    func: |args, _, _, _| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Ok(args[0].clone())
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

pub const ALL_MACROS: [Value; 9] = [
    Value::Macro(DEF),
    Value::Macro(QUOTE),
    Value::Macro(TIME),
    Value::Macro(DO),
    Value::Macro(CONST),
    Value::Macro(SET),
    Value::Macro(IF),
    Value::Macro(WHILE),
    Value::Macro(LET),
];

// TODO:
// SliceMacro,
// FnMacro,
// SwitchMacro,
// ForMacro,
// WhileMacro,
// BreakMacro,
// ContinueMacro,
// EnumMacro,
// StructMacro,
// MacroMacro,
// ClassMacro,
// ThreadMacro,
