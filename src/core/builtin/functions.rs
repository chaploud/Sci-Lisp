/* core/builtin/functions.rs */

use std::borrow::Cow;

#[allow(unused_imports)]
use crate::core::types::error::{arity_error, arity_error_range, arity_error_min, Error};
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


pub const ALL_FUNCTIONS: [Value; 8] = [
    Value::Function(TYPE),
    Value::Function(PRINT),
    Value::Function(ADD),
    Value::Function(SUB),
    Value::Function(MUL),
    Value::Function(DIV),
    Value::Function(FLOORDIV),
    Value::Function(REM),
];

// TODO:
// doc
