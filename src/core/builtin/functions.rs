/* core/builtin/functions.rs */

use std::borrow::Cow;

use crate::core::types::function::Function;
use crate::core::value::Value;

// NOTE: use Cow::Owned(name) to create a Function

pub const TYPE_FN: Function = Function {
    name: Cow::Borrowed("type"),
    func: |args: Vec<Value>| Value::type_name(&args[0])
};

pub const ALL_FUNCTIONS: [Value; 1] = [Value::Function(TYPE_FN)];
