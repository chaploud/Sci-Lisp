
use crate::core::environment::Environment;
use crate::core::value::Value;

pub fn eval(env: &mut Environment, value: Value) -> Result<Value, String> {
    Ok(Value::Nil)
}
