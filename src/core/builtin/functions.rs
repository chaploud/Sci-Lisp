/* core/builtin/functions.rs */

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::{fmt, ptr};

use once_cell::sync::Lazy;
use unescape;

use crate::core::builtin::generators::Range;
use crate::core::types::error::{arity_error, arity_error_min, type_error};
use crate::core::types::error::{arity_error_range, Result};
use crate::core::types::function::Function;
use crate::core::types::meta::Meta;
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
        doc: Cow::Borrowed(
            "Subtracts all remaining values from the first value. (- x) returns -x.",
        ),
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
pub static SYMBOL_EQUAL: Lazy<Symbol> = Lazy::new(|| {
    Symbol {
        name: Cow::Borrowed("="),
        meta: Meta {
            doc: Cow::Borrowed(
                "Returns true if all values are equal to each other and false otherwise. (= x) returns true.",
            ),
            mutable: false,
        },
        hash: fxhash::hash("=")
    }
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
pub static SYMBOL_NOTEQUAL: Lazy<Symbol> = Lazy::new(|| {
    Symbol {
    name: Cow::Borrowed("!="),
    meta: Meta {
        doc: Cow::Borrowed(
            "Returns false if all values are equal to each other and true otherwise. (!= x) returns false.",
        ),
        mutable: false,
    },
    hash: fxhash::hash("!=")
}
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

// is
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
pub static SYMBOL_GE: Lazy<Symbol> = Lazy::new(|| {
    Symbol {
    name: Cow::Borrowed(">="),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if all left values are greater than or equal to the right value. (>= x) returns true."),
        mutable: false,
    },
    hash: fxhash::hash(">=")
    }
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
        doc: Cow::Borrowed(
            "Returns true if all left values are greater than the right value. (> x) returns true.",
        ),
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
pub static SYMBOL_LE: Lazy<Symbol> = Lazy::new(|| {
    Symbol {
    name: Cow::Borrowed("<="),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if all left values are less than or equal to the right value. (<= x) returns true."),
        mutable: false,
    },
    hash: fxhash::hash("<=")
}
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
        doc: Cow::Borrowed(
            "Returns true if all left values are less than the right value. (< x) returns true.",
        ),
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
            Value::Vector(vector) => vector
                .value
                .first()
                .map_or(Ok(Value::Nil), |v| Ok(v.clone())),
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

        Ok(Value::Generator(Rc::new(RefCell::new(Range::new(
            start, end, step,
        )))))
    }
}

impl fmt::Display for RangeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function: range>")
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
