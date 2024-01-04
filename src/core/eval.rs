/* core/eval.rs */

use crate::core::builtin::macros::{SYMBOL_SYNTAX_QUOTING, SYMBOL_UNQUOTING};
use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::list::List;
use crate::core::types::map::Map;
use crate::core::types::set::Set;
use crate::core::types::vector::Vector;
use crate::core::value::Value;

pub fn splice(value: Value) -> Vec<Value> {
    match value {
        Value::Splicing(s) => {
            let mut res: Vec<Value> = vec![];
            for v in s {
                res.push(v);
            }
            res
        }
        _ => vec![value],
    }
}

pub fn ast_eval(
    environment: &mut Environment,
    ast: &mut Vec<Value>,
    value: Value,
) -> Result<Value> {
    ast.push(value.clone());
    eval(environment, ast)
}

pub fn ast_eval_with_splice(
    environment: &mut Environment,
    ast: &mut Vec<Value>,
    value: Value,
) -> Vec<Result<Value>> {
    match value {
        Value::Splicing(s) => {
            let mut res: Vec<Result<Value>> = vec![];
            for v in s {
                res.push(Ok(v));
            }
            res
        }
        _ => {
            ast.push(value.clone());
            vec![eval(environment, ast)]
        }
    }
}

pub fn eval_list(
    environment: &mut Environment,
    ast: &mut Vec<Value>,
    list: &List,
) -> Result<Value> {
    let first = match list.value.first() {
        None => return Ok(Value::List(list.clone())),
        Some(first) => first,
    };

    let first = match first {
        Value::Symbol(sym) => environment.get(sym)?.clone(),
        _ => return Err(Error::Syntax(format!("cannot call '{}'", first))),
    };

    let rest: Vec<Value> = list.value[1..].to_vec();

    let result: Result<Value> = match first {
        Value::Function(func) => {
            let args: Result<Vec<Value>> = rest
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect();
            func.call(args?)
        }
        Value::Macro(mac) => mac.call(rest, environment, ast, eval),
        _ => Err(Error::Syntax(format!("cannot call '{}'", first))),
    };

    result
}

pub fn eval(environment: &mut Environment, ast: &mut Vec<Value>) -> Result<Value> {
    let val = match ast.pop() {
        Some(val) => val,
        None => Value::Nil,
    };

    let in_syntax_quote = match environment.get(&SYMBOL_SYNTAX_QUOTING) {
        Ok(_) => true,
        Err(_) => false,
    };

    let in_unquoting = match environment.get(&SYMBOL_UNQUOTING) {
        Ok(_) => true,
        Err(_) => false,
    };

    match val {
        Value::Nil
        | Value::Bool(_)
        | Value::I64(_)
        | Value::F64(_)
        | Value::Regex(_)
        | Value::String(_) => Ok(val),
        Value::Symbol(symbol) => {
            if in_syntax_quote && !in_unquoting {
                return Ok(Value::Symbol(symbol));
            }
            Ok(environment.get(&symbol)?.clone())
        }
        Value::Keyword(_) => Ok(val),
        Value::List(list) => eval_list(environment, ast, &list),
        Value::Vector(vector) => {
            let result: Result<Vec<Value>> = vector
                .value
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect();
            Ok(Value::Vector(Vector::from(result?)))
        }
        Value::Map(map) => {
            let result: Result<Vec<(Value, Value)>> = map
                .value
                .into_iter()
                .map(|(k, v)| {
                    { ast_eval(environment, ast, k) }
                        .and_then(|ek| ast_eval(environment, ast, v).map(|ev| (ek, ev)))
                })
                .collect();

            Ok(Value::Map(Map::from(result?)))
        }
        Value::Set(set) => {
            let result: Result<Vec<Value>> = set
                .value
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect();
            Ok(Value::Set(Set::from(result?)))
        }
        _ => unreachable!(),
    }
}
