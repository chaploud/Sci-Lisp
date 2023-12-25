/* core/builtin/functions.rs */

use crate::core::types::function::Function;

pub const TYPE_FN: Function = Function {
    name: "type",
    call: |args| args[0].type_name(),
};
