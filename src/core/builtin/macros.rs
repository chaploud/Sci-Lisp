/* core/builtin/macros.rs */

use std::borrow::Cow;

use crate::core::types::error::Error;
#[allow(unused_imports)]
use crate::core::types::error::{arity_error, arity_error_range};
use crate::core::types::meta::Meta;
use crate::core::types::r#macro::Macro;
use crate::core::value::Value;

pub const DEF: Macro = Macro {
    name: Cow::Borrowed("def"),
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

        environment.put(symbol.name.clone(), value)?;
        Ok(Value::Symbol(symbol))
    },
    meta: Meta {
        doc: Cow::Borrowed("Bind a value to a symbol."),
        mutable: false,
    },
};

pub const QUOTE: Macro = Macro {
    name: Cow::Borrowed("quote"),
    func: |args, _, _, _| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Ok(args[0].clone())
    },
    meta: Meta {
        doc: Cow::Borrowed("Quote a value."),
        mutable: false,
    },
};

pub const TIME: Macro = Macro {
    name: Cow::Borrowed("time"),
    func: |args, environment, ast, evalfn| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let start = std::time::Instant::now();
        ast.push(args[1].clone());
        let result = evalfn(environment, ast)?;

        let end = std::time::Instant::now();
        println!("Elapsed time: {:?}", end - start);

        Ok(result)
    },
    meta: Meta {
        doc: Cow::Borrowed("Time the evaluation of an expression."),
        mutable: false,
    },
};

// pub const DO: Macro = Macro {
//     name: Cow::Borrowed("do"),
// }

pub const ALL_MACROS: [Value; 3] = [Value::Macro(DEF), Value::Macro(QUOTE), Value::Macro(TIME)];

// TODO:
// ConstMacro,
// SetMacro,
// SliceMacro,
// LetMacro,
// DoMacro,
// FnMacro,
// IfMacro,
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
