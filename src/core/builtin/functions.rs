/* core/builtin/functions.rs */

use std::borrow::Cow;
use std::ptr;

use crate::core::types::error::Result;
#[allow(unused_imports)]
use crate::core::types::error::{arity_error, arity_error_min, arity_error_range, Error};
use crate::core::types::function::Function;
use crate::core::types::meta::Meta;
use crate::core::types::symbol::Symbol;
use crate::core::value::Value;

// type
pub const SYMBOL_TYPE: Symbol = Symbol {
    name: Cow::Borrowed("type"),
    meta: Meta {
        doc: Cow::Borrowed("Get the type of a value"),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeFn;

impl Function for TypeFn {
    fn name(&self) -> Symbol {
        SYMBOL_TYPE
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        Value::type_name(&args[0])
    }
}

// print
pub const SYMBOL_PRINT: Symbol = Symbol {
    name: Cow::Borrowed("print"),
    meta: Meta {
        doc: Cow::Borrowed("Print a value."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintFn;

impl Function for PrintFn {
    fn name(&self) -> Symbol {
        SYMBOL_PRINT
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
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

// add(+)
pub const SYMBOL_ADD: Symbol = Symbol {
    name: Cow::Borrowed("+"),
    meta: Meta {
        doc: Cow::Borrowed("Add all arguments."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddFn;

impl Function for AddFn {
    fn name(&self) -> Symbol {
        SYMBOL_ADD
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
        let mut result: Value = Value::I64(0);
        for arg in args {
            result = result + arg;
        }
        Ok(result)
    }
}

// sub(-)
pub const SYMBOL_SUB: Symbol = Symbol {
    name: Cow::Borrowed("-"),
    meta: Meta {
        doc: Cow::Borrowed("Subtract all arguments."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubFn;

impl Function for SubFn {
    fn name(&self) -> Symbol {
        SYMBOL_SUB
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
        let mut result: Value = Value::I64(0);
        for arg in args {
            result = result - arg;
        }
        Ok(result)
    }
}

// mul(*)
pub const SYMBOL_MUL: Symbol = Symbol {
    name: Cow::Borrowed("*"),
    meta: Meta {
        doc: Cow::Borrowed("Multiply all arguments."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MulFn;

impl Function for MulFn {
    fn name(&self) -> Symbol {
        SYMBOL_MUL
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
        let mut result: Value = Value::I64(1);
        for arg in args {
            result = result * arg;
        }
        Ok(result)
    }
}

// div(/)
pub const SYMBOL_DIV: Symbol = Symbol {
    name: Cow::Borrowed("/"),
    meta: Meta {
        doc: Cow::Borrowed("Divide all arguments."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DivFn;

impl Function for DivFn {
    fn name(&self) -> Symbol {
        SYMBOL_DIV
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
        let mut result: Value = Value::F64(1.0);
        for arg in args {
            result = result / arg;
        }
        Ok(result)
    }
}

// floordiv(//)
pub const SYMBOL_FLOORDIV: Symbol = Symbol {
    name: Cow::Borrowed("//"),
    meta: Meta {
        doc: Cow::Borrowed("Divide and floor all arguments."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloorDivFn;

impl Function for FloorDivFn {
    fn name(&self) -> Symbol {
        SYMBOL_FLOORDIV
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
        let mut result: Value = Value::I64(1);
        for arg in args {
            result = result.floor_div(arg);
        }
        Ok(result)
    }
}

// rem(%)
pub const SYMBOL_REM: Symbol = Symbol {
    name: Cow::Borrowed("%"),
    meta: Meta {
        doc: Cow::Borrowed("Remainder of two arguments."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemFn;

impl Function for RemFn {
    fn name(&self) -> Symbol {
        SYMBOL_REM
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() < 2 || args.len() > 2 {
            return Err(arity_error(2, args.len()));
        }
        let result = args[0].clone() % args[1].clone();
        Ok(result)
    }
}

// equal(=)
pub const SYMBOL_EQUAL: Symbol = Symbol {
    name: Cow::Borrowed("="),
    meta: Meta {
        doc: Cow::Borrowed("Check if all arguments are equal."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualFn;

impl Function for EqualFn {
    fn name(&self) -> Symbol {
        SYMBOL_EQUAL
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
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
        doc: Cow::Borrowed("Check if all arguments are not equal."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotEqualFn;

impl Function for NotEqualFn {
    fn name(&self) -> Symbol {
        SYMBOL_NOTEQUAL
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
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
    fn name(&self) -> Symbol {
        SYMBOL_IS
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 2 {
            return Err(arity_error(2, args.len()));
        }
        Ok(Value::Bool(ptr::eq(&args[0], &args[1])))
    }
}

// ge(>=)
pub const SYMBOL_GE: Symbol = Symbol {
    name: Cow::Borrowed(">="),
    meta: Meta {
        doc: Cow::Borrowed("Greater than or equal to."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeFn;

impl Function for GeFn {
    fn name(&self) -> Symbol {
        SYMBOL_GE
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
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
        doc: Cow::Borrowed("Greater than."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GtFn;

impl Function for GtFn {
    fn name(&self) -> Symbol {
        SYMBOL_GT
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
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
        doc: Cow::Borrowed("Less than or equal to."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeFn;

impl Function for LeFn {
    fn name(&self) -> Symbol {
        SYMBOL_LE
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
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
        doc: Cow::Borrowed("Less than."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LtFn;

impl Function for LtFn {
    fn name(&self) -> Symbol {
        SYMBOL_LT
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
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

// doc
pub const SYMBOL_DOC: Symbol = Symbol {
    name: Cow::Borrowed("doc"),
    meta: Meta {
        doc: Cow::Borrowed("Get the documentation of a value."),
        mutable: false,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocFn;

impl Function for DocFn {
    fn name(&self) -> Symbol {
        SYMBOL_DOC
    }

    fn call(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != 1 {
            return Err(arity_error(1, args.len()));
        }

        let result = match &args[0] {
            Value::Function(func) => func.name().meta.doc.clone(),
            Value::Macro(mac) => mac.name().meta.doc.clone(),
            Value::Symbol(sym) => sym.meta.doc.clone(),
            _ => Cow::from(format!("{} has no documentation.", args[0].type_name()?)),
        };

        println!("{}", result);

        Ok(Value::Nil)
    }
}
