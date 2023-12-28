/* core/builtin/functions.rs */

use std::borrow::Cow;

#[allow(unused_imports)]
use crate::core::types::error::{arity_error, arity_error_range};
use crate::core::types::function::Function;
use crate::core::types::meta::Meta;
use crate::core::value::Value;

pub const TYPE: Function = Function {
    name: Cow::Borrowed("type"),
    func: |args: Vec<Value>| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::type_name(&args[0])
    },
    meta: Meta {
        doc: Cow::Borrowed("Get the type of a value."),
        mutable: false,
    },
};

pub const ALL_FUNCTIONS: [Value; 1] = [Value::Function(TYPE)];

// TODO:
// doc
