/* core/eval.rs */

use crate::core::environment::Environment;
use crate::core::types::error::Result;
use crate::core::types::list::List;
use crate::core::types::map::Map;
use crate::core::types::set::Set;
use crate::core::types::vector::Vector;
use crate::core::value::Evaluable;
use crate::core::value::Value;

pub fn eval(environment: &mut Environment, ast: &mut Vec<Value>) -> Result<Value> {
    let val = match ast.pop() {
        Some(val) => val,
        None => Value::Nil,
    };

    match val {
        Value::Nil => Ok(val),
        Value::Bool(_) => Ok(val),
        Value::I64(_) => Ok(val),
        Value::F64(_) => Ok(val),
        Value::Regex(_) => Ok(val),
        Value::String(_) => Ok(val),
        Value::Symbol(symbol) => Ok(symbol.eval(environment)?),
        Value::Keyword(_) => Ok(val),
        Value::List(list) => {
            let result: Result<Vec<Value>> = list
                .value
                .into_iter()
                .map(|v| {
                    ast.push(v);
                    eval(environment, ast)
                })
                .collect();
            let result_list = List::from(result?);
            Ok(result_list.eval(environment)?)
        }
        Value::Vector(vector) => {
            let result: Result<Vec<Value>> = vector
                .value
                .into_iter()
                .map(|v| {
                    ast.push(v);
                    eval(environment, ast)
                })
                .collect();
            Ok(Value::Vector(Vector::from(result?)))
        }
        Value::Map(map) => {
            let result: Result<Vec<(Value, Value)>> = map
                .value
                .into_iter()
                .map(|(k, v)| {
                    {
                        ast.push(k);
                        eval(environment, ast)
                    }.and_then(|ek| {
                        ast.push(v);
                        eval(environment, ast).map(|ev| (ek, ev))
                    })
                })
                .collect();
            Ok(Value::Map(Map::from(result?)))
        }
        Value::Set(set) => {
            let result: Result<Vec<Value>> = set
                .value
                .into_iter()
                .map(|v| {
                    ast.push(v);
                    eval(environment, ast)
                })
                .collect();
            Ok(Value::Set(Set::from(result?)))
        }
        // TODO: function, macro, error
        _ => unreachable!(),
    }
}
