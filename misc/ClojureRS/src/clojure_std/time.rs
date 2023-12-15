use crate::error_message;
use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use std::time::{SystemTime, UNIX_EPOCH};

/// provides a function to return current time in nanoseconds
#[derive(Debug, Clone)]
pub struct NanoTimeFn {}
impl ToValue for NanoTimeFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}

impl IFn for NanoTimeFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.is_empty() {
            let ns = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            Value::F64(ns as f64)
        } else {
            error_message::wrong_arg_count(0, args.len());
            Value::Nil
        }
    }
}
