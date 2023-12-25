/* core/builtin/macros.rs */

use std::borrow::Cow;

use crate::core::types::r#macro::Macro;
use crate::core::value::Value;

pub const DEF: Macro = Macro {
    name: Cow::Borrowed("def"),
    func: |args: Vec<Value>| Ok(Value::Nil),
};

pub const ALL_MACROS: [Value; 1] = [Value::Macro(DEF)];
