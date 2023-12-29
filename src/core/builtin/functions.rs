/* core/builtin/functions.rs */

use std::borrow::Cow;
use std::ptr;

#[allow(unused_imports)]
use crate::core::types::error::{arity_error, arity_error_min, arity_error_range, Error};
use crate::core::types::function::Function;
use crate::core::types::meta::Meta;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

pub const TYPE: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("type"),
        meta: Meta {
            doc: Cow::Borrowed("Get the type of a value."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::type_name(&args[0])
    },
};

pub const PRINT: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("print"),
        meta: Meta {
            doc: Cow::Borrowed("Print a value."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        for (n, arg) in args.into_iter().enumerate() {
            if n > 0 {
                print!(" ");
            }
            print!("{}", arg);
        }
        println!("");
        Ok(Value::Nil)
    },
};

pub const ADD: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("+"),
        meta: Meta {
            doc: Cow::Borrowed("Add all arguments."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        let mut result: Value = Value::I64(0);
        for arg in args {
            result = result + arg;
        }
        Ok(result)
    },
};

pub const SUB: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("-"),
        meta: Meta {
            doc: Cow::Borrowed("Subtract all arguments."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        let mut result: Value = Value::I64(0);
        for arg in args {
            result = result - arg;
        }
        Ok(result)
    },
};

pub const MUL: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("*"),
        meta: Meta {
            doc: Cow::Borrowed("Multiply all arguments."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        let mut result: Value = Value::I64(1);
        for arg in args {
            result = result * arg;
        }
        Ok(result)
    },
};

pub const DIV: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("/"),
        meta: Meta {
            doc: Cow::Borrowed("Divide all arguments as f64"),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        if args.len() < 1 {
            return Err(arity_error_min(1, args.len()));
        }
        let mut result: Value = Value::F64(1.0);
        result = result / args[0].clone();
        if args.len() == 1 {
            return Ok(result);
        }
        for arg in args.into_iter().skip(1) {
            result = result / arg;
        }
        Ok(result)
    },
};

pub const FLOORDIV: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("//"),
        meta: Meta {
            doc: Cow::Borrowed("Divide and floor all arguments."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        if args.len() < 1 {
            return Err(arity_error_min(1, args.len()));
        }
        let mut result: Value = Value::I64(1);
        if args.len() == 1 {
            return Ok(result.floor_div(args[0].clone()));
        }
        result = args[0].clone();
        for arg in args.into_iter().skip(1) {
            result = result.floor_div(arg);
        }
        Ok(result)
    },
};

pub const REM: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("%"),
        meta: Meta {
            doc: Cow::Borrowed("Remainder of two arguments."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        if args.len() < 2 || args.len() > 2 {
            return Err(arity_error(2, args.len()));
        }
        let result = args[0].clone() % args[1].clone();
        Ok(result)
    },
};

pub const EQUAL: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("="),
        meta: Meta {
            doc: Cow::Borrowed("Check if all arguments are equal."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
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
    },
};

pub const NOTEQUAL: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("!="),
        meta: Meta {
            doc: Cow::Borrowed("Check if all arguments are not equal."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
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
    },
};

pub const IS: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("is"),
        meta: Meta {
            doc: Cow::Borrowed("Check if two values are the same."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }
        Ok(Value::Bool(ptr::eq(&args[0], &args[1])))
    },
};

pub const GE: Function = Function {
    name: Symbol {
        name: Cow::Borrowed(">="),
        meta: Meta {
            doc: Cow::Borrowed("Greater than or equal to."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
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
    },
};

pub const GT: Function = Function {
    name: Symbol {
        name: Cow::Borrowed(">"),
        meta: Meta {
            doc: Cow::Borrowed("Greater than."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
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
    },
};

pub const LE: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("<="),
        meta: Meta {
            doc: Cow::Borrowed("Less than or equal to."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
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
    },
};

pub const LT: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("<"),
        meta: Meta {
            doc: Cow::Borrowed("Less than."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
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
    },
};

pub const DOC: Function = Function {
    name: Symbol {
        name: Cow::Borrowed("doc"),
        meta: Meta {
            doc: Cow::Borrowed("Get the documentation of a value."),
            mutable: false,
        },
    },
    func: |args: Vec<Value>| {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let result = match &args[0] {
            Value::Function(func) => func.name.meta.doc.clone(),
            Value::Macro(mac) => mac.name.meta.doc.clone(),
            Value::Symbol(sym) => sym.meta.doc.clone(),
            _ => Cow::Owned(format!("{} has no documentation.", args[0].type_name()?)),
        };

        println!("{}", result);

        Ok(Value::Nil)
    },
};

pub const ALL_FUNCTIONS: [Value; 16] = [
    Value::Function(TYPE),
    Value::Function(PRINT),
    Value::Function(ADD),
    Value::Function(SUB),
    Value::Function(MUL),
    Value::Function(DIV),
    Value::Function(FLOORDIV),
    Value::Function(REM),
    Value::Function(EQUAL),
    Value::Function(NOTEQUAL),
    Value::Function(IS),
    Value::Function(GE),
    Value::Function(GT),
    Value::Function(LE),
    Value::Function(LT),
    Value::Function(DOC),
];

// TODO:
// doc => macro
