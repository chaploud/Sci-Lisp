/* core/builtin/functions.rs */

use crate::core::types::function::Function;

pub const TYPE_FN: Function =  Function {
    name: "type".to_string(),
    call: |args| {
        args[0].type_name()
    }
};
