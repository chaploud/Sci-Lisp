/* core/eval.rs */

use std::cell::RefCell;
use std::rc::Rc;

use crate::core::builtin::macros::{SYMBOL_SYNTAX_QUOTING, SYMBOL_UNQUOTING};
use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::list::List;
use crate::core::value::Value;

use super::types::function::Function;
use super::types::slice::Slice;

pub fn is_need_eval(environment: &Rc<RefCell<Environment>>) -> bool {
    let in_syntax_quote = environment.borrow().get(SYMBOL_SYNTAX_QUOTING).is_ok();

    let in_unquoting = environment.borrow().get(SYMBOL_UNQUOTING).is_ok();

    !in_syntax_quote || in_unquoting
}

pub fn splicing_insert(values: Vec<Value>) -> Vec<Value> {
    values
        .into_iter()
        .flat_map(|v| match v {
            Value::Splicing(s) => s.value,
            _ => vec![v],
        })
        .collect()
}

pub fn ast_eval(
    environment: &Rc<RefCell<Environment>>,
    ast: &mut Vec<Value>,
    value: Value,
) -> Result<Value> {
    ast.push(value.clone());
    eval(environment, ast)
}

pub fn eval_rest(
    environment: &Rc<RefCell<Environment>>,
    ast: &mut Vec<Value>,
    rest: Vec<Value>,
) -> Result<Vec<Value>> {
    let result: Vec<Value> = rest
        .into_iter()
        .map(|v| ast_eval(environment, ast, v))
        .collect::<Result<Vec<Value>>>()?;

    Ok(splicing_insert(result))
}

pub fn eval_list(
    environment: &Rc<RefCell<Environment>>,
    ast: &mut Vec<Value>,
    list: &List,
) -> Result<Value> {
    let first = match list.value.first() {
        None => return Ok(Value::List(list.clone())),
        Some(first) => first,
    };

    let first = match first {
        Value::Symbol(sym) => environment.borrow().get(sym.clone())?.1.clone(),
        Value::List(list) => ast_eval(environment, ast, Value::List(list.clone()))?,
        Value::Vector(v) => ast_eval(environment, ast, Value::Vector(v.clone()))?,
        Value::Splicing(_) => return Err(Error::Syntax("splicing in function call".to_string())),
        f => f.clone(),
    };

    let rest: Vec<Value> = list.value[1..].to_vec();

    let result: Result<Value> = match first {
        Value::Function(func) => func.borrow_mut().call(eval_rest(environment, ast, rest)?),
        Value::I64(mut int) => int.call(eval_rest(environment, ast, rest)?),
        Value::String(mut s) => s.call(eval_rest(environment, ast, rest)?),
        Value::Keyword(mut k) => k.call(eval_rest(environment, ast, rest)?),
        Value::Vector(v) => v.call(eval_rest(environment, ast, rest)?),
        Value::Macro(mac) => mac.borrow_mut().call(rest, environment, ast, eval),
        f => {
            if is_need_eval(environment) {
                return Err(Error::Syntax(format!("cannot call '{}'", f)));
            }
            let fst: Value = ast_eval(environment, ast, f)?;
            let mut ret: Vec<Value> = rest
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect::<Result<Vec<Value>>>()?;

            ret = splicing_insert(ret);

            ret.insert(0, fst);
            Value::as_list(ret)
        }
    };

    result
}

pub fn eval(environment: &Rc<RefCell<Environment>>, ast: &mut Vec<Value>) -> Result<Value> {
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
        Value::Slice(s) => {
            let start = ast_eval(environment, ast, s.start.clone())?;
            let end = ast_eval(environment, ast, s.end.clone())?;
            let step = ast_eval(environment, ast, s.step.clone())?;
            match (&start, &end, &step) {
                (Value::I64(_), Value::I64(_), Value::I64(_)) => {}
                _ => return Err(Error::Type("slice can contain only i64".to_string())),
            }
            Ok(Value::Slice(Rc::new(Slice::new(start, end, step))))
        }
        Value::Symbol(symbol) => {
            if !is_need_eval(environment) {
                return Ok(Value::Symbol(symbol));
            }
            Ok(environment.borrow().get(symbol)?.1.clone())
        }
        Value::List(list) => eval_list(environment, ast, &list),
        Value::Vector(vector) => {
            let mut result: Vec<Value> = vector
                .value
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect::<Result<Vec<Value>>>()?;

            result = splicing_insert(result);

            Value::as_vector(result)
        }
        Value::Map(map) => {
            let result: Vec<(Value, Value)> = map
                .value
                .into_iter()
                .map(|(k, v)| {
                    {
                        match k {
                            Value::Splicing(_) => {
                                return Err(Error::Syntax("splicing in map".to_string()))
                            }
                            _ => ast_eval(environment, ast, k),
                        }
                    }
                    .and_then(|ek| match v {
                        Value::Splicing(_) => Err(Error::Syntax("splicing in map".to_string())),
                        _ => ast_eval(environment, ast, v).map(|ev| (ek, ev)),
                    })
                })
                .collect::<Result<Vec<(Value, Value)>>>()?;

            Value::as_map(result)
        }
        Value::Set(set) => {
            let mut result: Vec<Value> = set
                .value
                .into_iter()
                .map(|v| ast_eval(environment, ast, v))
                .collect::<Result<Vec<Value>>>()?;

            result = splicing_insert(result);

            Value::as_set(result)
        }
        _ => unreachable!(),
    }
}
