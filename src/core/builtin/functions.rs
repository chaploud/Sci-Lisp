/* core/builtin/functions.rs */

use std::borrow::Cow;
use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

use crate::core::environment::Environment;
use crate::core::types::error::Result;
use crate::core::types::error::{arity_error, arity_error_min, type_error};
use crate::core::types::function::Function;
use crate::core::types::meta::Meta;
use crate::core::types::symbol::Symbol;
use crate::core::types::vector::Vector;
use crate::core::value::Value;

// type
pub const SYMBOL_TYPE: Symbol = Symbol {
    name: Cow::Borrowed("type"),
    meta: Meta {
        doc: Cow::Borrowed("Get the type of a value."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeFn;

impl Function for TypeFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Ok(Value::String(Value::type_name(&args[0])))
    }
}

// print
pub const SYMBOL_PRINT: Symbol = Symbol {
    name: Cow::Borrowed("print"),
    meta: Meta {
        doc: Cow::Borrowed("Print value(s) to stdout."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintFn;

impl Function for PrintFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        for (n, arg) in args.into_iter().enumerate() {
            if n > 0 {
                print!(" ");
            }
            print!("{}", arg);
        }
        println!("");
        Ok(Value::Nil)
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
pub const SYMBOL_ADD: Symbol = Symbol {
    name: Cow::Borrowed("+"),
    meta: Meta {
        doc: Cow::Borrowed("Adds all values. (+) returns 0."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddFn;

impl Function for AddFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        let mut result: Value = Value::I64(0);
        for arg in args {
            helper_is_number(arg.clone())?;
            result = result + arg;
        }
        Ok(result)
    }
}

// sub(-)
pub const SYMBOL_SUB: Symbol = Symbol {
    name: Cow::Borrowed("-"),
    meta: Meta {
        doc: Cow::Borrowed(
            "Subtracts all remaining values from the first value. (- x) returns -x.",
        ),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubFn;

impl Function for SubFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 1 {
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

// mul(*)
pub const SYMBOL_MUL: Symbol = Symbol {
    name: Cow::Borrowed("*"),
    meta: Meta {
        doc: Cow::Borrowed("Multiplies all values. (*) returns 1."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MulFn;

impl Function for MulFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        let mut result: Value = Value::I64(1);
        for arg in args {
            helper_is_number(arg.clone())?;
            result = result * arg;
        }
        Ok(result)
    }
}

// div(/)
pub const SYMBOL_DIV: Symbol = Symbol {
    name: Cow::Borrowed("/"),
    meta: Meta {
        doc: Cow::Borrowed("Divide the first value by all remaining values."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DivFn;

impl Function for DivFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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

// floordiv(//)
pub const SYMBOL_FLOORDIV: Symbol = Symbol {
    name: Cow::Borrowed("//"),
    meta: Meta {
        doc: Cow::Borrowed("Divide the first value by all remaining values and floor."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloorDivFn;

impl Function for FloorDivFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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

// rem(%)
pub const SYMBOL_REM: Symbol = Symbol {
    name: Cow::Borrowed("%"),
    meta: Meta {
        doc: Cow::Borrowed("Remainder of the first value divided by the second value."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemFn;

impl Function for RemFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 2 || args.len() > 2 {
            return Err(arity_error(2, args.len()));
        }

        helper_is_number(args[0].clone())?;
        helper_is_number(args[1].clone())?;

        let result = args[0].clone() % args[1].clone();
        Ok(result)
    }
}

// equal(=)
pub const SYMBOL_EQUAL: Symbol = Symbol {
    name: Cow::Borrowed("="),
    meta: Meta {
        doc: Cow::Borrowed(
            "Returns true if all values are equal to each other and false otherwise. (= x) returns true.",
        ),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualFn;

impl Function for EqualFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 1 {
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

// notequal(!=)
pub const SYMBOL_NOTEQUAL: Symbol = Symbol {
    name: Cow::Borrowed("!="),
    meta: Meta {
        doc: Cow::Borrowed(
            "Returns false if all values are equal to each other and true otherwise. (!= x) returns false.",
        ),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotEqualFn;

impl Function for NotEqualFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 1 {
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

// is
pub const SYMBOL_IS: Symbol = Symbol {
    name: Cow::Borrowed("is"),
    meta: Meta {
        doc: Cow::Borrowed("Check if two values are the same."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsFn;

impl Function for IsFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }
        // TODO: This returns almost always false
        // Pythonic 'is' would be better
        Ok(Value::Bool(ptr::eq(&args[0], &args[1])))
    }
}

// ge(>=)
pub const SYMBOL_GE: Symbol = Symbol {
    name: Cow::Borrowed(">="),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if all left values are greater than or equal to the right value. (>= x) returns true."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeFn;

impl Function for GeFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 1 {
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

// gt(>)
pub const SYMBOL_GT: Symbol = Symbol {
    name: Cow::Borrowed(">"),
    meta: Meta {
        doc: Cow::Borrowed(
            "Returns true if all left values are greater than the right value. (> x) returns true.",
        ),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GtFn;

impl Function for GtFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 1 {
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

// le(<=)
pub const SYMBOL_LE: Symbol = Symbol {
    name: Cow::Borrowed("<="),
    meta: Meta {
        doc: Cow::Borrowed("Returns true if all left values are less than or equal to the right value. (<= x) returns true."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeFn;

impl Function for LeFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 1 {
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

// lt(<)
pub const SYMBOL_LT: Symbol = Symbol {
    name: Cow::Borrowed("<"),
    meta: Meta {
        doc: Cow::Borrowed(
            "Returns true if all left values are less than the right value. (< x) returns true.",
        ),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LtFn;

impl Function for LtFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() < 1 {
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

// str
pub const SYMBOL_STR: Symbol = Symbol {
    name: Cow::Borrowed("str"),
    meta: Meta {
        doc: Cow::Borrowed("Convert a value to a string."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrFn;

impl Function for StrFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::to_str(&args[0])
    }
}

// i64
pub const SYMBOL_I64: Symbol = Symbol {
    name: Cow::Borrowed("i64"),
    meta: Meta {
        doc: Cow::Borrowed("Convert a value to an i64."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I64Fn;

impl Function for I64Fn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::to_i64(&args[0])
    }
}

// f64
pub const SYMBOL_F64: Symbol = Symbol {
    name: Cow::Borrowed("f64"),
    meta: Meta {
        doc: Cow::Borrowed("Convert a value to an f64."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct F64Fn;

impl Function for F64Fn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::to_f64(&args[0])
    }
}

// first
pub const SYMBOL_FIRST: Symbol = Symbol {
    name: Cow::Borrowed("first"),
    meta: Meta {
        doc: Cow::Borrowed("Get the first element of a collection."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstFn;

impl Function for FirstFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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

// rest
pub const SYMBOL_REST: Symbol = Symbol {
    name: Cow::Borrowed("rest"),
    meta: Meta {
        doc: Cow::Borrowed("Get the rest of a collection."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestFn;

impl Function for RestFn {
    fn call(&self, args: Vec<Value>, _environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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
                Value::as_set(
                    set.value[1..]
                        .into_iter()
                        .map(|v| v.clone())
                        .collect::<Vec<Value>>(),
                )
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

// TODO:
// isinstance
// format
// read
// write
