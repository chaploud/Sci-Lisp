/* core/builtin/functions.rs */

use std::borrow::Cow;

#[allow(unused_imports)]
use crate::core::types::error::{arity_error, arity_error_range, Error};
use crate::core::types::function::Function;
use crate::core::types::meta::Meta;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

pub const TYPE: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("type"),
        meta: Meta {
            doc: Cow::Borrowed("Get the type of a value."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::type_name(&args[0])
    },
};

pub const PRINT: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("print"),
        meta: Meta {
            doc: Cow::Borrowed("Print a value."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        for (n, arg) in args.into_iter().enumerate() {
            if n > 0 {
                print!(" ");
            }
            print!("{}", arg);
        }
        println!("");
        Ok(Value::Nil)
    },
};

pub const ALL_FUNCTIONS: [Value; 2] = [Value::Function(TYPE), Value::Function(PRINT)];

// TODO:
// doc
