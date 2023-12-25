/* core/builtin/macros.rs */

use crate::core::types::r#macro::Macro;

pub const DEF_MACRO: Macro = Macro {
    name: "def",
    call: |args| args[0].type_name(),
};
