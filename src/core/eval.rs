/* core/eval.rs */

use crate::core::builtin::macros::{SYMBOL_SYNTAX_QUOTING, SYMBOL_UNQUOTING};
use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::list::List;
use crate::core::value::Value;

pub fn is_need_eval(environment: &mut Environment) -> bool {
    let in_syntax_quote = match environment.get(&SYMBOL_SYNTAX_QUOTING) {
        Ok(_) => true,
        Err(_) => false,
    };

    let in_unquoting = match environment.get(&SYMBOL_UNQUOTING) {
        Ok(_) => true,
        Err(_) => false,
    };

    return !in_syntax_quote || in_unquoting;
}

pub fn ast_eval(
    environment: &mut Environment,
    ast: &mut Vec<Value>,
    value: Value,
) -> Result<Value> {
    ast.push(value.clone());
    return eval(environment, ast);
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
        Value::Symbol(sym) => environment.get(sym)?.1.clone(),
        f => f.clone(),
    };

    let rest: Vec<Value> = list.value[1..].to_vec();

    let result: Result<Value> = match first {
        Value::Function(func) => {
            let ret: Result<Vec<Value>> = rest
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect();

            func.call(ret?)
        }
        Value::Macro(mac) => mac.call(rest, environment, ast, eval),
        f => {
            if is_need_eval(environment) {
                return Err(Error::Syntax(format!("cannot call '{}'", f)));
            }
            let fst: Value = ast_eval(environment, ast, f)?;
            let mut ret: Vec<Value> = rest
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect::<Result<Vec<Value>>>()?;

            ret.insert(0, fst);
            Value::as_list(ret)
        }
    };

    result
}

pub fn eval(environment: &mut Environment, ast: &mut Vec<Value>) -> Result<Value> {
    let val = match ast.pop() {
        Some(val) => val,
        None => Value::Nil,
    };

    match val {
        Value::Nil
        | Value::Bool(_)
        | Value::I64(_)
        | Value::F64(_)
        | Value::Regex(_)
        | Value::String(_)
        | Value::Keyword(_) => Ok(val),
        Value::Symbol(symbol) => {
            if !is_need_eval(environment) {
                return Ok(Value::Symbol(symbol));
            }
            Ok(environment.get(&symbol)?.1.clone())
        }
        Value::List(list) => eval_list(environment, ast, &list),
        Value::Vector(vector) => {
            let result: Vec<Value> = vector
                .value
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect::<Result<Vec<Value>>>()?;

            Value::as_vector(result)
        }
        Value::Map(map) => {
            let result: Vec<(Value, Value)> = map
                .value
                .into_iter()
                .map(|(k, v)| {
                    { ast_eval(environment, ast, k) }
                        .and_then(|ek| ast_eval(environment, ast, v).map(|ev| (ek, ev)))
                })
                .collect::<Result<Vec<(Value, Value)>>>()?;

            Value::as_map(result)
        }
        Value::Set(set) => {
            let result: Vec<Value> = set
                .value
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect::<Result<Vec<Value>>>()?;

            Value::as_set(result)
        }
        _ => unreachable!(),
    }
}
