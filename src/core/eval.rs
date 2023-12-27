/* core/eval.rs */

use std::borrow::Cow;

use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::list::List;
use crate::core::types::map::Map;
use crate::core::types::set::Set;
use crate::core::types::vector::Vector;
use crate::core::value::Evaluable;
use crate::core::value::Value;

pub fn eval_list(
    environment: &mut Environment,
    ast: &mut Vec<Value>,
    list: &List,
) -> Result<Value> {
    let first = match list.value.first() {
        None => return Ok(Value::List(list.clone())),
        Some(first) => first,
    };

    let rest = list.value[1..].to_vec();

    let first = match first {
        Value::Symbol(sym) => environment.get(&sym.name)?.clone(),
        _ => return Err(Error::Syntax(format!("cannot call '{}'", first))),
    };

    let result: Result<Value> = match first {
        Value::Function(func) => {
            let args: Result<Vec<Value>> = rest
                .into_iter()
                .map(|v| {
                    ast.push(v.clone());
                    eval(environment, ast)
                })
                .collect();
            func.call(args?)
        }
        Value::Macro(mac) => match mac.name {
            Cow::Borrowed("def") => {
                let mut args: Vec<Value> = vec![];
                for (i, v) in rest.into_iter().enumerate() {
                    if i == 0 {
                        args.push(v);
                    } else {
                        ast.push(v);
                        args.push(eval(environment, ast)?);
                    }
                }
                mac.call(args, environment)
            }

            Cow::Borrowed("quote") => mac.call(rest, environment),

            _ => mac.call(rest, environment),
        },
        _ => Err(Error::Syntax(format!("cannot call '{}'", first))),
    };

    result
}

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
        Value::List(list) => eval_list(environment, ast, &list),
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
            if map.value.len() % 2 != 0 {
                return Err(Error::Syntax(
                    "map must contain an even number of forms".to_string(),
                ));
            }
            let result: Result<Vec<(Value, Value)>> = map
                .value
                .into_iter()
                .map(|(k, v)| {
                    {
                        ast.push(k);
                        eval(environment, ast)
                    }
                    .and_then(|ek| {
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
