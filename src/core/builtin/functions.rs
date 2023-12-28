/* core/builtin/functions.rs */

use std::borrow::Cow;

use crate::core::types::function::Function;
use crate::core::value::Value;
use crate::core::types::error::{arity_error, arity_error_range};

// NOTE: use Cow::Owned(name) to create a Function

pub const TYPE: Function = Function {
    name: Cow::Borrowed("type"),
    func: |args: Vec<Value>| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::type_name(&args[0])
    }
};

pub const ALL_FUNCTIONS: [Value; 1] = [Value::Function(TYPE)];
