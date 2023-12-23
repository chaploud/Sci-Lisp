use crate::core::environment::Environment;
use crate::core::list::List;
use crate::core::map::Map;
use crate::core::set::Set;
use crate::core::value::Value;
use crate::core::vector::Vector;

pub fn eval(environment: &mut Environment, ast: &mut Vec<Value>) -> Result<Value, String> {
    let val = match ast.pop() {
        Some(val) => val,
        None => Value::Nil,
    };

    match val {
        Value::SpecialForm(_) => Ok(val),
        Value::Nil => Ok(val),
        Value::Bool(_) => Ok(val),
        Value::I64(_) => Ok(val),
        Value::F64(_) => Ok(val),
        Value::Regex(_) => Ok(val),
        Value::String(_) => Ok(val),
        Value::Symbol(symbol) => {
            match environment.get(&symbol.value) {
                Ok(value) => Ok(*value),
                Err(err) => Err(err.to_string()),
            }
        },
        Value::Keyword(_) => Ok(val),
        Value::List(list) => {
            let result: Vec<Value> = list
                .value
                .into_iter()
                .map(|v| {
                    ast.push(v);
                    eval(environment, ast).unwrap()
                })
                .collect();
            Ok(Value::List(List::from(result)))
        }
        Value::Vector(vector) => {
            let result: Vec<Value> = vector
                .value
                .into_iter()
                .map(|v| {
                    ast.push(v);
                    eval(environment, ast).unwrap()
                })
                .collect();
            Ok(Value::Vector(Vector::from(result)))
        }
        Value::Map(map) => {
            let result: Vec<(Value, Value)> = map
                .value
                .into_iter()
                .map(|(k, v)| {
                    (
                        {
                            ast.push(k);
                            eval(environment, ast).unwrap()
                        },
                        {
                            ast.push(v);
                            eval(environment, ast).unwrap()
                        },
                    )
                })
                .collect();
            Ok(Value::Map(Map::from(result)))
        }
        Value::Set(set) => {
            let result: Vec<Value> = set
                .value
                .into_iter()
                .map(|v| {
                    ast.push(v);
                    eval(environment, ast).unwrap()
                })
                .collect();
            Ok(Value::Set(Set::from(result)))
        }
        // Value::Function(_) => Ok(value),
        _ => unreachable!(),
    }
}
