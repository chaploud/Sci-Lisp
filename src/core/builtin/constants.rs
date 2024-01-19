/* core/builtin/constants.rs */

use std::borrow::Cow;

use once_cell::sync::Lazy;

use crate::core::types::meta::Meta;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

pub static SYMBOL_PI: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("*pi*"),
    meta: Meta {
        doc: Cow::Borrowed("Get a random integer between two numbers."),
        mutable: false,
    },
    hash: fxhash::hash("*pi*"),
});
pub const CONST_PI: Value = Value::F64(std::f64::consts::PI);

pub static SYMBOL_E: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("*e*"),
    meta: Meta {
        doc: Cow::Borrowed("Get a random integer between two numbers."),
        mutable: false,
    },
    hash: fxhash::hash("*e*"),
});
pub const CONST_E: Value = Value::F64(std::f64::consts::E);
