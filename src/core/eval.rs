use crate::core::environment::Environment;
use crate::core::value::Value;

pub fn eval(environment: &mut Environment, ast: &mut Vec<Value>) -> Result<Value, String> {
    let value = match ast.pop() {
        Some(value) => value,
        None => return Ok(Value::Nil),
    };

    match value {
        Value::Nil => Ok(value),
        Value::Bool(_) => Ok(value),
        Value::I64(_) => Ok(value),
        Value::F64(_) => Ok(value),
        Value::Regex(_) => Ok(value),
        Value::String(_) => Ok(value),
        // Value::Symbol(symbol) => {
        //     match environment.get(&symbol.value) {
        //         Ok(value) => Ok(*value),
        //         Err(err) => Err(err.to_string()),
        //     }
        // },
        // Value::Keyword(_) => Ok(value),
        // Value::List(_) => Ok(value),
        // Value::Vector(_) => Ok(value),
        // Value::Map(_) => Ok(value),
        // Value::Set(_) => Ok(value),
        // Value::Function(_) => Ok(value),
        _ => unreachable!(),
    }
}
