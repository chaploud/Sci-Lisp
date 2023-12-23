use crate::core::environment::Environment;
use crate::core::value::Value;

fn ast_pop(ast: &mut Vec<Value>) -> Value {
    match ast.pop() {
        Some(val) => val,
        None => Value::Nil,
    }
}

pub fn eval(environment: &mut Environment, ast: &mut Vec<Value>) -> Result<Value, String> {
    let val = ast_pop(ast);

    match val {
        Value::Nil => Ok(val),
        Value::Bool(_) => Ok(val),
        Value::I64(_) => Ok(val),
        Value::F64(_) => Ok(val),
        Value::Regex(_) => Ok(val),
        Value::String(_) => Ok(val),
        // Value::Symbol(symbol) => {
        //     match environment.get(&symbol.value) {
        //         Ok(value) => Ok(*value),
        //         Err(err) => Err(err.to_string()),
        //     }
        // },
        // Value::Keyword(_) => Ok(value),
        Value::List(list) => {
            let mut result = list.clone();
            list.value.into_iter().enumerate().for_each(|(n, v)| {
                ast.push(v);
                result.value[n] = eval(environment, ast).unwrap();
            });
            Ok(Value::List(result))
        }
        // Value::Vector(_) => Ok(value),
        // Value::Map(_) => Ok(value),
        // Value::Set(_) => Ok(value),
        // Value::Function(_) => Ok(value),
        _ => unreachable!(),
    }
}
