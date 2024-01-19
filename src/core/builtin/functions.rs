/* core/builtin/functions.rs */

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::{fmt, ptr};

use once_cell::sync::Lazy;
use rand::Rng;
use unescape;

use crate::core::builtin::generators::Range;
use crate::core::types::error::Error;
use crate::core::types::error::{arity_error, arity_error_min, type_error};
use crate::core::types::error::{arity_error_range, Result};
use crate::core::types::function::Function;
use crate::core::types::list::List;
use crate::core::types::meta::Meta;
use crate::core::types::sliceable::Sliceable;
use crate::core::types::symbol::Symbol;
use crate::core::types::vector::Vector;
use crate::core::value::Value;

// type
pub static SYMBOL_TYPE: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("type"),
    meta: Meta {
        doc: Cow::Borrowed("Get the type of a value."),
        mutable: false,
    },
    hash: fxhash::hash("type"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeFn;

impl Function for TypeFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Ok(Value::String(Value::type_name(&args[0])))
    }
}

impl fmt::Display for TypeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: type>")
    }
}

// print
pub static SYMBOL_PRINT: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("print"),
    meta: Meta {
        doc: Cow::Borrowed("Print value(s) to stdout."),
        mutable: false,
    },
    hash: fxhash::hash("print"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintFn;

impl Function for PrintFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        for (n, arg) in args.into_iter().enumerate() {
            if n > 0 {
                print!(" ");
            }
            if let Value::String(s) = arg {
                print!("{}", unescape::unescape(&s).unwrap());
            } else {
                print!("{}", arg);
            }
        }
        println!();
        Ok(Value::Nil)
    }
}

impl fmt::Display for PrintFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: print>")
    }
}

// inc
pub static SYMBOL_INC: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("inc"),
    meta: Meta {
        doc: Cow::Borrowed("Increment a value by 1."),
        mutable: false,
    },
    hash: fxhash::hash("inc"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncFn;

impl Function for IncFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::I64(i + 1)),
            Value::F64(f) => Ok(Value::F64(f + 1.0)),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for IncFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: inc>")
    }
}

// dec
pub static SYMBOL_DEC: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("dec"),
    meta: Meta {
        doc: Cow::Borrowed("Decrement a value by 1."),
        mutable: false,
    },
    hash: fxhash::hash("dec"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecFn;

impl Function for DecFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::I64(i - 1)),
            Value::F64(f) => Ok(Value::F64(f - 1.0)),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for DecFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: dec>")
    }
}

fn helper_is_number(arg: Value) -> Result<Value> {
    match arg {
        Value::I64(i) => Ok(Value::I64(i)),
        Value::F64(f) => Ok(Value::F64(f)),
        _ => Err(type_error("i64 or f64", arg.type_name().as_str())),
    }
}

// add(+)
pub static SYMBOL_ADD: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("+"),
    meta: Meta {
        doc: Cow::Borrowed("Adds all values. (+) returns 0."),
        mutable: false,
    },
    hash: fxhash::hash("+"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddFn;

impl Function for AddFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        let mut result: Value = Value::I64(0);
        for arg in args {
            helper_is_number(arg.clone())?;
            result = result + arg;
        }
        Ok(result)
    }
}

impl fmt::Display for AddFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: + >")
    }
}

// sub(-)
pub static SYMBOL_SUB: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("-"),
    meta: Meta {
        doc: Cow::Borrowed("Subtracts all remaining values from the first value. (- x) returns -x."),
        mutable: false,
    },
    hash: fxhash::hash("-"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubFn;

impl Function for SubFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }

        if args.len() == 1 {
            helper_is_number(args[0].clone())?;
            return Ok(-args[0].clone());
        }

        let mut result = args[0].clone();
        for arg in args[1..].iter() {
            helper_is_number(arg.clone())?;
            result = result - arg.clone();
        }
        Ok(result)
    }
}

impl fmt::Display for SubFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: - >")
    }
}

// mul(*)
pub static SYMBOL_MUL: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("*"),
    meta: Meta {
        doc: Cow::Borrowed("Multiplies all values. (*) returns 1."),
        mutable: false,
    },
    hash: fxhash::hash("*"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MulFn;

impl Function for MulFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        let mut result: Value = Value::I64(1);
        for arg in args {
            helper_is_number(arg.clone())?;
            result = result * arg;
        }
        Ok(result)
    }
}

impl fmt::Display for MulFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: * >")
    }
}

// div(/)
pub static SYMBOL_DIV: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("/"),
    meta: Meta {
        doc: Cow::Borrowed("Divide the first value by all remaining values."),
        mutable: false,
    },
    hash: fxhash::hash("/"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DivFn;

impl Function for DivFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() < 2 {
            return Err(arity_error_min(2, args.len()));
        }

        let mut result = args[0].clone();
        helper_is_number(result.clone())?;
        for arg in args[1..].iter() {
            helper_is_number(arg.clone())?;
            result = result / arg.clone();
        }
        Ok(result)
    }
}

impl fmt::Display for DivFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: / >")
    }
}

// floordiv(//)
pub static SYMBOL_FLOORDIV: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("//"),
    meta: Meta {
        doc: Cow::Borrowed("Divide the first value by all remaining values and floor."),
        mutable: false,
    },
    hash: fxhash::hash("//"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloorDivFn;

impl Function for FloorDivFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() < 2 {
            return Err(arity_error_min(2, args.len()));
        }

        let mut result = args[0].clone();
        helper_is_number(result.clone())?;
        for arg in args[1..].iter() {
            helper_is_number(arg.clone())?;
            result = result.floor_div(arg.clone());
        }
        Ok(result)
    }
}

impl fmt::Display for FloorDivFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: // >")
    }
}

// rem(%)
pub static SYMBOL_REM: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("%"),
    meta: Meta {
        doc: Cow::Borrowed("Remainder of the first value divided by the second value."),
        mutable: false,
    },
    hash: fxhash::hash("%"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemFn;

impl Function for RemFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() < 2 || args.len() > 2 {
            return Err(arity_error(2, args.len()));
        }

        helper_is_number(args[0].clone())?;
        helper_is_number(args[1].clone())?;

        let result = args[0].clone() % args[1].clone();
        Ok(result)
    }
}

impl fmt::Display for RemFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: % >")
    }
}

// equal(=)
pub static SYMBOL_EQUAL: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("="),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if all values are equal to each other and false otherwise. (= x) returns true."),
        mutable: false,
    },
    hash: fxhash::hash("="),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualFn;

impl Function for EqualFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }
        if args.len() == 1 {
            return Ok(Value::Bool(true));
        }
        let mut prev = args[0].clone();
        for arg in args.into_iter().skip(1) {
            if arg != prev {
                return Ok(Value::Bool(false));
            }
            prev = arg;
        }
        Ok(Value::Bool(true))
    }
}

impl fmt::Display for EqualFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: = >")
    }
}

// notequal(!=)
pub static SYMBOL_NOTEQUAL: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("!="),
    meta: Meta {
        doc: Cow::Borrowed("Returns false if all values are equal to each other and true otherwise. (!= x) returns false."),
        mutable: false,
    },
    hash: fxhash::hash("!="),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotEqualFn;

impl Function for NotEqualFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }
        if args.len() == 1 {
            return Ok(Value::Bool(false));
        }
        let mut prev = args[0].clone();
        for arg in args.into_iter().skip(1) {
            if arg == prev {
                return Ok(Value::Bool(false));
            }
            prev = arg;
        }
        Ok(Value::Bool(true))
    }
}

impl fmt::Display for NotEqualFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: != >")
    }
}

// is TODO: This returns almost always false. need another approach
pub static SYMBOL_IS: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("is"),
    meta: Meta {
        doc: Cow::Borrowed("Check if two values are the same."),
        mutable: false,
    },
    hash: fxhash::hash("is"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsFn;

impl Function for IsFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }
        // TODO: This returns almost always false
        // Pythonic 'is' would be better
        Ok(Value::Bool(ptr::eq(&args[0], &args[1])))
    }
}

impl fmt::Display for IsFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: is>")
    }
}

// ge(>=)
pub static SYMBOL_GE: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed(">="),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if all left values are greater than or equal to the right value. (>= x) returns true."),
        mutable: false,
    },
    hash: fxhash::hash(">="),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeFn;

impl Function for GeFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }
        if args.len() == 1 {
            return Ok(Value::Bool(true));
        }
        let mut prev = args[0].clone();
        for arg in args.into_iter().skip(1) {
            if prev < arg {
                return Ok(Value::Bool(false));
            }
            prev = arg;
        }
        Ok(Value::Bool(true))
    }
}

impl fmt::Display for GeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: >= >")
    }
}

// gt(>)
pub static SYMBOL_GT: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed(">"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if all left values are greater than the right value. (> x) returns true."),
        mutable: false,
    },
    hash: fxhash::hash(">"),
});
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GtFn;

impl Function for GtFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }
        if args.len() == 1 {
            return Ok(Value::Bool(true));
        }
        let mut prev = args[0].clone();
        for arg in args.into_iter().skip(1) {
            if prev <= arg {
                return Ok(Value::Bool(false));
            }
            prev = arg;
        }
        Ok(Value::Bool(true))
    }
}

impl fmt::Display for GtFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: > >")
    }
}

// le(<=)
pub static SYMBOL_LE: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("<="),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if all left values are less than or equal to the right value. (<= x) returns true."),
        mutable: false,
    },
    hash: fxhash::hash("<="),
});
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeFn;

impl Function for LeFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }
        if args.len() == 1 {
            return Ok(Value::Bool(true));
        }
        let mut prev = args[0].clone();
        for arg in args.into_iter().skip(1) {
            if prev > arg {
                return Ok(Value::Bool(false));
            }
            prev = arg;
        }
        Ok(Value::Bool(true))
    }
}

impl fmt::Display for LeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: <= >")
    }
}

// lt(<)
pub static SYMBOL_LT: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("<"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if all left values are less than the right value. (< x) returns true."),
        mutable: false,
    },
    hash: fxhash::hash("<"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LtFn;

impl Function for LtFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.is_empty() {
            return Err(arity_error_min(1, args.len()));
        }
        if args.len() == 1 {
            return Ok(Value::Bool(true));
        }
        let mut prev = args[0].clone();
        for arg in args.into_iter().skip(1) {
            if prev >= arg {
                return Ok(Value::Bool(false));
            }
            prev = arg;
        }
        Ok(Value::Bool(true))
    }
}

impl fmt::Display for LtFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: < >")
    }
}

// xor
pub static SYMBOL_XOR: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("xor"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if one of the values is truthy and the other is falsy."),
        mutable: false,
    },
    hash: fxhash::hash("xor"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XorFn;

impl Function for XorFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }
        Ok(Value::Bool(args[0].is_truthy() ^ args[1].is_truthy()))
    }
}

impl fmt::Display for XorFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: xor>")
    }
}

// not
pub static SYMBOL_NOT: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("not"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is falsy or nil."),
        mutable: false,
    },
    hash: fxhash::hash("not"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotFn;

impl Function for NotFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }
        Ok(Value::Bool(!args[0].is_truthy()))
    }
}

impl fmt::Display for NotFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: not>")
    }
}

// zero?
pub static SYMBOL_ZEROQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("zero?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is zero."),
        mutable: false,
    },
    hash: fxhash::hash("zero?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZeroQFn;

impl Function for ZeroQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }
        match args[0].clone() {
            Value::I64(i) => Ok(Value::Bool(i == 0)),
            Value::F64(f) => Ok(Value::Bool(f == 0.0)),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for ZeroQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: zero?>")
    }
}

// nil?
pub static SYMBOL_NILQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("nil?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is nil."),
        mutable: false,
    },
    hash: fxhash::hash("nil?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NilQFn;

impl Function for NilQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }
        Ok(Value::Bool(args[0].is_nil()))
    }
}

impl fmt::Display for NilQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: nil?>")
    }
}

// true?
pub static SYMBOL_TRUEQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("true?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is true."),
        mutable: false,
    },
    hash: fxhash::hash("true?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrueQFn;

impl Function for TrueQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }
        Ok(Value::Bool(args[0].is_true()))
    }
}

impl fmt::Display for TrueQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: true?>")
    }
}

// false?
pub static SYMBOL_FALSEQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("false?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is false."),
        mutable: false,
    },
    hash: fxhash::hash("false?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalseQFn;

impl Function for FalseQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }
        Ok(Value::Bool(args[0].is_false()))
    }
}

impl fmt::Display for FalseQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: false?>")
    }
}

// number?
pub static SYMBOL_NUMBERQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("number?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is a number."),
        mutable: false,
    },
    hash: fxhash::hash("number?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberQFn;

impl Function for NumberQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_number()))
    }
}

impl fmt::Display for NumberQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: number?>")
    }
}

// i64?
pub static SYMBOL_I64Q: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("i64?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is an i64."),
        mutable: false,
    },
    hash: fxhash::hash("i64?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I64QFn;

impl Function for I64QFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_i64()))
    }
}

impl fmt::Display for I64QFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: i64?>")
    }
}

// f64?
pub static SYMBOL_F64Q: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("f64?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is an f64."),
        mutable: false,
    },
    hash: fxhash::hash("f64?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct F64QFn;

impl Function for F64QFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_f64()))
    }
}

impl fmt::Display for F64QFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: f64?>")
    }
}

// even?
pub static SYMBOL_EVENQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("even?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is an even number."),
        mutable: false,
    },
    hash: fxhash::hash("even?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvenQFn;

impl Function for EvenQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        match args[0].clone() {
            Value::I64(i) => Ok(Value::Bool(i % 2 == 0)),
            _ => Err(type_error("i64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for EvenQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: even?>")
    }
}

// odd?
pub static SYMBOL_ODDQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("odd?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is an odd number."),
        mutable: false,
    },
    hash: fxhash::hash("odd?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OddQFn;

impl Function for OddQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        match args[0].clone() {
            Value::I64(i) => Ok(Value::Bool(i % 2 != 0)),
            _ => Err(type_error("i64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for OddQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: odd?>")
    }
}

// empty?
pub static SYMBOL_EMPTYQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("empty?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is empty."),
        mutable: false,
    },
    hash: fxhash::hash("empty?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyQFn;

impl Function for EmptyQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_empty()))
    }
}

impl fmt::Display for EmptyQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: empty?>")
    }
}

// string?
pub static SYMBOL_STRINGQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("string?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is a string."),
        mutable: false,
    },
    hash: fxhash::hash("string?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringQFn;

impl Function for StringQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_string()))
    }
}

impl fmt::Display for StringQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: string?>")
    }
}

// keyword?
pub static SYMBOL_KEYWORDQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("keyword?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is a keyword."),
        mutable: false,
    },
    hash: fxhash::hash("keyword?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordQFn;

impl Function for KeywordQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_keyword()))
    }
}

impl fmt::Display for KeywordQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: keyword?>")
    }
}

// symbol?
pub static SYMBOL_SYMBOLQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("symbol?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is a symbol."),
        mutable: false,
    },
    hash: fxhash::hash("symbol?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolQFn;

impl Function for SymbolQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_symbol()))
    }
}

impl fmt::Display for SymbolQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: symbol?>")
    }
}

// list?
pub static SYMBOL_LISTQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("list?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is a list."),
        mutable: false,
    },
    hash: fxhash::hash("list?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListQFn;

impl Function for ListQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_list()))
    }
}

impl fmt::Display for ListQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: list?>")
    }
}

// vector?
pub static SYMBOL_VECTORQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("vector?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is a vector."),
        mutable: false,
    },
    hash: fxhash::hash("vector?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorQFn;

impl Function for VectorQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_vector()))
    }
}

impl fmt::Display for VectorQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: vector?>")
    }
}

// map?
pub static SYMBOL_MAPQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("map?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is a map."),
        mutable: false,
    },
    hash: fxhash::hash("map?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapQFn;

impl Function for MapQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_map()))
    }
}

impl fmt::Display for MapQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: map?>")
    }
}

// set?
pub static SYMBOL_SETQ: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("set?"),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if the value is a set."),
        mutable: false,
    },
    hash: fxhash::hash("set?"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetQFn;

impl Function for SetQFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }
        Ok(Value::Bool(args[0].is_set()))
    }
}

impl fmt::Display for SetQFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: set?>")
    }
}

// str
pub static SYMBOL_STR: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("str"),
    meta: Meta {
        doc: Cow::Borrowed("Convert a value to a string."),
        mutable: false,
    },
    hash: fxhash::hash("str"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrFn;

impl Function for StrFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::to_str(&args[0])
    }
}

impl fmt::Display for StrFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: str>")
    }
}

// i64
pub static SYMBOL_I64: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("i64"),
    meta: Meta {
        doc: Cow::Borrowed("Convert a value to an i64."),
        mutable: false,
    },
    hash: fxhash::hash("i64"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I64Fn;

impl Function for I64Fn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::to_i64(&args[0])
    }
}

impl fmt::Display for I64Fn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: i64>")
    }
}

// f64
pub static SYMBOL_F64: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("f64"),
    meta: Meta {
        doc: Cow::Borrowed("Convert a value to an f64."),
        mutable: false,
    },
    hash: fxhash::hash("f64"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct F64Fn;

impl Function for F64Fn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::to_f64(&args[0])
    }
}

impl fmt::Display for F64Fn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: f64>")
    }
}

// list
pub static SYMBOL_LIST: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("list"),
    meta: Meta {
        doc: Cow::Borrowed("Cast to a list."),
        mutable: false,
    },
    hash: fxhash::hash("list"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListFn;

impl Function for ListFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        match args.len() {
            0 => Value::as_list(vec![]),
            1 => match args[0].clone() {
                Value::List(l) => Ok(Value::List(l)),
                Value::Vector(v) => Value::as_list(v.value),
                Value::Map(m) => {
                    let mut list = vec![];
                    for (k, v) in m.value {
                        list.push(Value::Vector(Vector { value: vec![k, v] }));
                    }
                    Value::as_list(list)
                }
                Value::Set(s) => {
                    let mut list = vec![];
                    for v in s.value {
                        list.push(v);
                    }
                    Value::as_list(list)
                }
                Value::String(s) => {
                    let mut list = vec![];
                    for c in s.chars() {
                        list.push(Value::String(c.to_string()));
                    }
                    Value::as_list(list)
                }
                _ => Ok(Value::List(List {
                    value: vec![args[0].clone()],
                })),
            },
            _ => {
                let mut list = vec![];
                for arg in args {
                    list.push(arg);
                }
                Value::as_list(list)
            }
        }
    }
}

impl fmt::Display for ListFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: list>")
    }
}

// vector
pub static SYMBOL_VECTOR: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("vector"),
    meta: Meta {
        doc: Cow::Borrowed("Cast to a vector."),
        mutable: false,
    },
    hash: fxhash::hash("vector"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorFn;

impl Function for VectorFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        match args.len() {
            0 => Value::as_vector(vec![]),
            1 => match args[0].clone() {
                Value::List(l) => Value::as_vector(l.value),
                Value::Vector(v) => Ok(Value::Vector(v)),
                Value::Map(m) => {
                    let mut vector = vec![];
                    for (k, v) in m.value {
                        vector.push(Value::Vector(Vector { value: vec![k, v] }));
                    }
                    Value::as_vector(vector)
                }
                Value::Set(s) => {
                    let mut vector = vec![];
                    for v in s.value {
                        vector.push(v);
                    }
                    Value::as_vector(vector)
                }
                Value::String(s) => {
                    let mut vector = vec![];
                    for c in s.chars() {
                        vector.push(Value::String(c.to_string()));
                    }
                    Value::as_vector(vector)
                }
                _ => Ok(Value::Vector(Vector {
                    value: vec![args[0].clone()],
                })),
            },
            _ => {
                let mut vector = vec![];
                for arg in args {
                    vector.push(arg);
                }
                Value::as_vector(vector)
            }
        }
    }
}

impl fmt::Display for VectorFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: vector>")
    }
}

// hmap
pub static SYMBOL_HMAP: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("hmap"),
    meta: Meta {
        doc: Cow::Borrowed("Cast to a map."),
        mutable: false,
    },
    hash: fxhash::hash("hmap"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HmapFn;

impl Function for HmapFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        match args.len() {
            0 => Value::as_map(vec![]),
            1 => match args[0].clone() {
                Value::List(l) => {
                    let mut map = vec![];
                    if l.len() % 2 != 0 {
                        return Err(Error::Value("hmap: list must have even number of elements".to_string()));
                    }
                    for chunk in l.value.chunks(2) {
                        map.push((chunk[0].clone(), chunk[1].clone()));
                    }
                    Value::as_map(map)
                }
                Value::Vector(v) => {
                    let mut map = vec![];
                    if v.len() % 2 != 0 {
                        return Err(Error::Value("hmap: vector must have even number of elements".to_string()));
                    }
                    for chunk in v.value.chunks(2) {
                        map.push((chunk[0].clone(), chunk[1].clone()));
                    }
                    Value::as_map(map)
                }
                Value::Map(m) => Ok(Value::Map(m)),
                _ => Err(Error::Value(
                    "hmap: argument must be list, vector, map, or even number of arguments".to_string(),
                )),
            },
            _ => {
                let mut map = vec![];
                if args.len() % 2 != 0 {
                    return Err(Error::Value(
                        "hmap: argument must be list, vector, map, set, or even number of arguments".to_string(),
                    ));
                }

                for chunk in args.chunks(2) {
                    map.push((chunk[0].clone(), chunk[1].clone()));
                }

                Value::as_map(map)
            }
        }
    }
}

impl fmt::Display for HmapFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: hmap>")
    }
}

// hset
pub static SYMBOL_HSET: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("hset"),
    meta: Meta {
        doc: Cow::Borrowed("Cast to a set."),
        mutable: false,
    },
    hash: fxhash::hash("hset"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HsetFn;

impl Function for HsetFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        match args.len() {
            0 => Value::as_set(vec![]),
            1 => match args[0].clone() {
                Value::List(l) => Value::as_set(l.value),
                Value::Vector(v) => Value::as_set(v.value),
                Value::Map(m) => {
                    let mut set = vec![];
                    for (k, _) in m.value {
                        set.push(k);
                    }
                    Value::as_set(set)
                }
                Value::Set(s) => Ok(Value::Set(s)),
                _ => Value::as_set(vec![args[0].clone()]),
            },
            _ => {
                let mut set = vec![];
                for arg in args {
                    set.push(arg);
                }
                Value::as_set(set)
            }
        }
    }
}

impl fmt::Display for HsetFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: hset>")
    }
}

// first
pub static SYMBOL_FIRST: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("first"),
    meta: Meta {
        doc: Cow::Borrowed("Get the first element of a collection."),
        mutable: false,
    },
    hash: fxhash::hash("first"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstFn;

impl Function for FirstFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        match args[0].clone() {
            Value::List(list) => list.value.first().map_or(Ok(Value::Nil), |v| Ok(v.clone())),
            Value::Vector(vector) => vector.value.first().map_or(Ok(Value::Nil), |v| Ok(v.clone())),
            Value::Map(map) => map.value.first().map_or(Ok(Value::Nil), |(k, v)| {
                Ok(Value::Vector(Vector {
                    value: vec![k.clone(), v.clone()],
                }))
            }),
            Value::Set(set) => set.value.first().map_or(Ok(Value::Nil), |v| Ok(v.clone())),
            Value::String(s) => {
                if s.is_empty() {
                    Ok(Value::Nil)
                } else {
                    Ok(Value::String(s[0..1].to_string()))
                }
            }
            _ => Err(type_error(
                "first: argument must be list, vector, map, set or string",
                args[0].type_name().as_str(),
            )),
        }
    }
}

impl fmt::Display for FirstFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: first>")
    }
}

// rest
pub static SYMBOL_REST: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("rest"),
    meta: Meta {
        doc: Cow::Borrowed("Get the rest of a collection."),
        mutable: false,
    },
    hash: fxhash::hash("rest"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestFn;

impl Function for RestFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        match args[0].clone() {
            Value::List(list) => {
                if list.value.is_empty() {
                    return Value::as_list(vec![]);
                }
                Value::as_list(list.value[1..].to_vec())
            }
            Value::Vector(vector) => {
                if vector.value.is_empty() {
                    return Value::as_vector(vec![]);
                }
                Value::as_vector(vector.value[1..].to_vec())
            }
            Value::Map(map) => {
                if map.value.is_empty() {
                    return Value::as_map(vec![]);
                }
                Value::as_map(
                    map.value[1..]
                        .into_iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect::<Vec<(Value, Value)>>(),
                )
            }
            Value::Set(set) => {
                if set.value.is_empty() {
                    return Value::as_set(vec![]);
                }
                Value::as_set(set.value[1..].into_iter().cloned().collect::<Vec<Value>>())
            }
            Value::String(s) => {
                if s.is_empty() {
                    Ok(Value::String("".to_string()))
                } else {
                    Ok(Value::String(s[1..].to_string()))
                }
            }
            _ => Err(type_error(
                "rest: argument must be list, vector, map, set or string",
                args[0].type_name().as_str(),
            )),
        }
    }
}

impl fmt::Display for RestFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: rest>")
    }
}

// range
pub static SYMBOL_RANGE: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("range"),
    meta: Meta {
        doc: Cow::Borrowed("Create a range of i64."),
        mutable: false,
    },
    hash: fxhash::hash("range"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeFn;

impl Function for RangeFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.is_empty() || args.len() > 3 {
            return Err(arity_error_range(1, 3, args.len()));
        }

        let mut start = 0;
        let mut end = 0;
        let mut step = 1;

        if args.len() == 1 {
            end = match args[0] {
                Value::I64(i) => i,
                _ => return Err(type_error("i64", args[0].type_name().as_str())),
            };
        } else if args.len() == 2 {
            start = match args[0] {
                Value::I64(i) => i,
                _ => return Err(type_error("i64", args[0].type_name().as_str())),
            };
            end = match args[1] {
                Value::I64(i) => i,
                _ => return Err(type_error("i64", args[1].type_name().as_str())),
            };
        } else if args.len() == 3 {
            start = match args[0] {
                Value::I64(i) => i,
                _ => return Err(type_error("i64", args[0].type_name().as_str())),
            };
            end = match args[1] {
                Value::I64(i) => i,
                _ => return Err(type_error("i64", args[1].type_name().as_str())),
            };
            step = match args[2] {
                Value::I64(i) => i,
                _ => return Err(type_error("i64", args[2].type_name().as_str())),
            };
        }

        Ok(Value::Generator(Rc::new(RefCell::new(Range::new(start, end, step)))))
    }
}

impl fmt::Display for RangeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: range>")
    }
}

// sqrt
pub static SYMBOL_SQRT: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("sqrt"),
    meta: Meta {
        doc: Cow::Borrowed("Get the square root of a number."),
        mutable: false,
    },
    hash: fxhash::hash("sqrt"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqrtFn;

impl Function for SqrtFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::F64((i as f64).sqrt())),
            Value::F64(f) => Ok(Value::F64(f.sqrt())),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for SqrtFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: sqrt>")
    }
}

// abs
pub static SYMBOL_ABS: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("abs"),
    meta: Meta {
        doc: Cow::Borrowed("Get the absolute value of a number."),
        mutable: false,
    },
    hash: fxhash::hash("abs"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbsFn;

impl Function for AbsFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error_range(1, 1, args.len()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::I64(i.abs())),
            Value::F64(f) => Ok(Value::F64(f.abs())),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for AbsFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: abs>")
    }
}

// cos
pub static SYMBOL_COS: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("cos"),
    meta: Meta {
        doc: Cow::Borrowed("Get the cosine of a number."),
        mutable: false,
    },
    hash: fxhash::hash("cos"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CosFn;

impl Function for CosFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(type_error("i64 or f64", args[0].type_name().as_str()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::F64((i as f64).cos())),
            Value::F64(f) => Ok(Value::F64(f.cos())),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for CosFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: cos>")
    }
}

// sin
pub static SYMBOL_SIN: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("sin"),
    meta: Meta {
        doc: Cow::Borrowed("Get the sine of a number."),
        mutable: false,
    },
    hash: fxhash::hash("sin"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SinFn;

impl Function for SinFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(type_error("i64 or f64", args[0].type_name().as_str()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::F64((i as f64).sin())),
            Value::F64(f) => Ok(Value::F64(f.sin())),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for SinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: sin>")
    }
}

// tan
pub static SYMBOL_TAN: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("tan"),
    meta: Meta {
        doc: Cow::Borrowed("Get the tangent of a number."),
        mutable: false,
    },
    hash: fxhash::hash("tan"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TanFn;

impl Function for TanFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(type_error("i64 or f64", args[0].type_name().as_str()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::F64((i as f64).tan())),
            Value::F64(f) => Ok(Value::F64(f.tan())),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for TanFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: tan>")
    }
}

// acos
pub static SYMBOL_ACOS: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("acos"),
    meta: Meta {
        doc: Cow::Borrowed("Get the arccosine of a number."),
        mutable: false,
    },
    hash: fxhash::hash("acos"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcosFn;

impl Function for AcosFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(type_error("i64 or f64", args[0].type_name().as_str()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::F64((i as f64).acos())),
            Value::F64(f) => Ok(Value::F64(f.acos())),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for AcosFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: acos>")
    }
}

// asin
pub static SYMBOL_ASIN: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("asin"),
    meta: Meta {
        doc: Cow::Borrowed("Get the arcsine of a number."),
        mutable: false,
    },
    hash: fxhash::hash("asin"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsinFn;

impl Function for AsinFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(type_error("i64 or f64", args[0].type_name().as_str()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::F64((i as f64).asin())),
            Value::F64(f) => Ok(Value::F64(f.asin())),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for AsinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: asin>")
    }
}

// atan
pub static SYMBOL_ATAN: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("atan"),
    meta: Meta {
        doc: Cow::Borrowed("Get the arctangent of a number."),
        mutable: false,
    },
    hash: fxhash::hash("atan"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtanFn;

impl Function for AtanFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(type_error("i64 or f64", args[0].type_name().as_str()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::F64((i as f64).atan())),
            Value::F64(f) => Ok(Value::F64(f.atan())),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for AtanFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: atan>")
    }
}

// log
pub static SYMBOL_LOG: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("log"),
    meta: Meta {
        doc: Cow::Borrowed("Get the natural logarithm of a number."),
        mutable: false,
    },
    hash: fxhash::hash("log"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogFn;

impl Function for LogFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }

        let base = match args[0].clone() {
            Value::I64(i) => i as f64,
            Value::F64(f) => f,
            _ => return Err(type_error("i64 or f64", args[1].type_name().as_str())),
        };

        match args[1].clone() {
            Value::I64(i) => Ok(Value::F64((i as f64).log(base))),
            Value::F64(f) => Ok(Value::F64(f.log(base))),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for LogFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: log>")
    }
}

// loge
pub static SYMBOL_LN: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("ln"),
    meta: Meta {
        doc: Cow::Borrowed("Get the natural logarithm of a number."),
        mutable: false,
    },
    hash: fxhash::hash("ln"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LnFn;

impl Function for LnFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::F64((i as f64).ln())),
            Value::F64(f) => Ok(Value::F64(f.ln())),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for LnFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: ln>")
    }
}

// log10
pub static SYMBOL_LOG10: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("log10"),
    meta: Meta {
        doc: Cow::Borrowed("Get the base 10 logarithm of a number."),
        mutable: false,
    },
    hash: fxhash::hash("log10"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Log10Fn;

impl Function for Log10Fn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        match args[0].clone() {
            Value::I64(i) => Ok(Value::F64((i as f64).log10())),
            Value::F64(f) => Ok(Value::F64(f.log10())),
            _ => Err(type_error("i64 or f64", args[0].type_name().as_str())),
        }
    }
}

impl fmt::Display for Log10Fn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<built-in function: log10>")
    }
}

// rand
pub static SYMBOL_RAND: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("rand"),
    meta: Meta {
        doc: Cow::Borrowed("Get a random number between 0 and 1."),
        mutable: false,
    },
    hash: fxhash::hash("rand"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandFn;

impl Function for RandFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if !args.is_empty() {
            return Err(arity_error(0, args.len()));
        }

        Ok(Value::F64(rand::random::<f64>()))
    }
}

impl fmt::Display for RandFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: rand>")
    }
}

// randint
pub static SYMBOL_RANDINT: Lazy<Symbol> = Lazy::new(|| Symbol {
    name: Cow::Borrowed("randint"),
    meta: Meta {
        doc: Cow::Borrowed("Get a random integer between two numbers."),
        mutable: false,
    },
    hash: fxhash::hash("randint"),
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandIntFn;

impl Function for RandIntFn {
    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }

        let start = match args[0].clone() {
            Value::I64(i) => i,
            _ => return Err(type_error("i64", args[0].type_name().as_str())),
        };

        let end = match args[1].clone() {
            Value::I64(i) => i,
            _ => return Err(type_error("i64", args[1].type_name().as_str())),
        };

        Ok(Value::I64(rand::thread_rng().gen_range(start..end)))
    }
}

impl fmt::Display for RandIntFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: randint>")
    }
}

// take
// collect
// map
// filter
// reduce
// zip
// apply
// partial
