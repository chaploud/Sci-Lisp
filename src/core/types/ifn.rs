use crate::core::types::error::Result;
use crate::core::value::Value;

pub trait IFn {
    fn call(&self, args: Vec<Value>) -> Result<Value>;
}
