use crate::ifn::IFn;
use crate::value::{ToValue, Value};
use std::rc::Rc;

use crate::error_message;

/// Primitive printing function; TODO: should be aware of *out*
/// (defn print-string [string] .. prints single string without linebreak.. )
#[derive(Debug, Clone)]
pub struct PrintStringFn {}
impl ToValue for PrintStringFn {
    fn to_value(&self) -> Value {
        Value::IFn(Rc::new(self.clone()))
    }
}
impl IFn for PrintStringFn {
    fn invoke(&self, args: Vec<Rc<Value>>) -> Value {
        if args.len() != 1 {
            return error_message::wrong_arg_count(1, args.len());
        }
        print!("{}", args.get(0).unwrap().to_string());
        Value::Nil
    }
}
