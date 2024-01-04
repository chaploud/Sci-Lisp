/* core/eval.rs */

use crate::core::builtin::macros::{SYMBOL_SYNTAX_QUOTING, SYMBOL_UNQUOTING};
use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::list::List;
use crate::core::types::map::Map;
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

pub fn splice_args(args: Vec<Value>) -> Result<Vec<Value>> {
    let mut ret = Vec::<Value>::new();
    for arg in args {
        if let Value::Splicing(spl) = arg {
            for s in spl {
                ret.push(s);
            }
            continue;
        }
        ret.push(arg);
    }
    Ok(ret)
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
        Value::Symbol(sym) => environment.get(sym)?.clone(),
        f => f.clone(),
    };

    let rest: Vec<Value> = list.value[1..].to_vec();

    let result: Result<Value> = match first {
        Value::Function(func) => {
            let ret: Result<Vec<Value>> = rest
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect();

            func.call(splice_args(ret?)?)
        }
        Value::Macro(mac) => mac.call(rest, environment, ast, eval),
        f => {
            if is_need_eval(environment) {
                return Err(Error::Syntax(format!("cannot call '{}'", f)));
            }
            let fst: Value = ast_eval(environment, ast, f)?;
            let ret: Result<Vec<Value>> = rest
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect();

            let mut args = splice_args(ret?)?;

            if let Value::Splicing(spl) = fst {
                let mut firsts = Vec::<Value>::new();
                for s in spl {
                    firsts.push(s);
                }
                firsts.extend(args);
                return Value::as_list(firsts);
            } else {
                args.insert(0, fst);
                return Value::as_list(args);
            }
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
        | Value::String(_) => Ok(val),
        Value::Symbol(symbol) => {
            if !is_need_eval(environment) {
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

            let ret = splice_args(result?)?;
            Value::as_vector(ret)
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

            let ret = splice_args(result?)?;
            Value::as_set(ret)
        }
        _ => unreachable!(),
    }
}
