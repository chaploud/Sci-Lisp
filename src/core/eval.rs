/* core/eval.rs */

use crate::core::builtin::macros::{SYMBOL_SYNTAX_QUOTING, SYMBOL_UNQUOTING, UNQUOTE_SPLICING};
use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::list::List;
use crate::core::types::map::Map;
use crate::core::types::set::Set;
use crate::core::types::vector::Vector;
use crate::core::value::Value;

pub fn ast_eval(
    environment: &mut Environment,
    ast: &mut Vec<Value>,
    value: Value,
) -> Result<Value> {
    ast.push(value.clone());
    eval(environment, ast)
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
            let ret: Result<Vec<Value>> = rest
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect();
            let mut args = Vec::<Value>::new();
            for v in ret? {
                if let Value::Splicing(spl) = v {
                    for s in spl {
                        args.push(s);
                    }
                    continue;
                }
                args.push(v);
            }
            func.call(args)
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
            let mut ret = Vector::new();
            for v in result? {
                if let Value::Splicing(spl) = v {
                    for s in spl {
                        ret.value.push(s);
                    }
                    continue;
                }
                ret.value.push(v);
            }
            Ok(Value::Vector(ret))
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

            let mut ret = Map::new();
            for (k, v) in result? {
                if let Value::Splicing(_) = k {
                    return Err(Error::Syntax(format!("cannot splice as a key in a map",)));
                }
                if let Value::Splicing(_) = v {
                    return Err(Error::Syntax(format!("cannot splice as a value in a map",)));
                }
                ret.value.insert(k, v);
            }

            Ok(Value::Map(ret))
        }
        Value::Set(set) => {
            let result: Result<Vec<Value>> = set
                .value
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect();
            let mut ret = Set::new();
            for v in result? {
                if let Value::Splicing(spl) = v {
                    for s in spl {
                        ret.value.insert(s);
                    }
                    continue;
                }
                ret.value.insert(v);
            }
            Ok(Value::Set(ret))
        }
        _ => unreachable!(),
    }
}
