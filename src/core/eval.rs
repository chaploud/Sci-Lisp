/* core/eval.rs */

use std::cell::RefCell;
use std::rc::Rc;

use crate::core::environment::Environment;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::function::Function;
use crate::core::types::list::List;
use crate::core::types::slice::Slice;
use crate::core::value::Value;

use super::builtin::macros::SYMBOL_UNQUOTE;
use super::builtin::macros::SYMBOL_UNQUOTE_SPLICING;

fn eval_rest(rest: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Vec<Value>> {
    let result: Vec<Value> = rest
        .into_iter()
        .map(|v| eval(v, environment.clone(), false))
        .collect::<Result<Vec<Value>>>()?;

    Ok(result)
}

fn eval_list(list: List, environment: Rc<RefCell<Environment>>, syntax_quote: bool) -> Result<Value> {
    let list_inner = list.value.clone();

    let first = match list_inner.first() {
        None => return Ok(Value::List(list.clone())),
        Some(f) => f,
    };

    let unquote = match first {
        Value::Symbol(sym) => *sym == *SYMBOL_UNQUOTE || *sym == *SYMBOL_UNQUOTE_SPLICING,
        _ => false,
    };

    if syntax_quote && !unquote {
        return Ok(Value::List(list.clone()));
    }

    let first: Value = match first {
        Value::Symbol(sym) => environment.borrow().get(sym)?,
        Value::List(list) => eval(Value::List(list.clone()), environment.clone(), syntax_quote)?,
        Value::Vector(v) => eval(Value::Vector(v.clone()), environment.clone(), syntax_quote)?,
        f => f.clone(),
    };

    let rest: Vec<Value> = list_inner[1..].to_vec();

    let result: Result<Value> = match first {
        Value::Function(func) => func.call(eval_rest(rest, environment)?),
        Value::I64(int) => int.call(eval_rest(rest, environment)?),
        Value::String(s) => s.call(eval_rest(rest, environment)?),
        Value::Keyword(k) => k.call(eval_rest(rest, environment)?),
        Value::Vector(v) => v.call(eval_rest(rest, environment)?),
        Value::Macro(mac) => mac.call(rest, environment),
        f => Err(Error::Syntax(format!("cannot call '{}'", f))),
    };

    result
}

pub fn eval_ast(ast: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Option<Value>> {
    let mut result = None;
    for expr in ast {
        result = Some(eval(expr, environment.clone(), false)?);
    }
    Ok(result)
}

pub fn eval(value: Value, environment: Rc<RefCell<Environment>>, syntax_quote: bool) -> Result<Value> {
    match value {
        Value::Nil
        | Value::Bool(_)
        | Value::I64(_)
        | Value::F64(_)
        | Value::Regex(_)
        | Value::String(_)
        | Value::Keyword(_)
        | Value::Function(_)
        | Value::Macro(_)
        | Value::ControlFlow(_)
        | Value::Generator(_) => Ok(value),
        Value::Slice(s) => {
            let start = eval(s.start.clone(), environment.clone(), syntax_quote)?;
            let end = eval(s.end.clone(), environment.clone(), syntax_quote)?;
            let step = eval(s.step.clone(), environment.clone(), syntax_quote)?;
            for v in vec![start.clone(), end.clone(), step.clone()] {
                match v {
                    Value::I64(_) | Value::Nil => {}
                    _ => return Err(Error::Type("slice can contain only i64 or nil".to_string())),
                }
            }
            Ok(Value::Slice(Rc::new(Slice::new(start, end, step))))
        }
        Value::Symbol(symbol) => {
            if syntax_quote {
                return Ok(Value::Symbol(symbol));
            }
            Ok(environment.borrow().get(&symbol)?)
        }
        Value::List(list) => eval_list(list, environment, syntax_quote),
        Value::Vector(vector) => {
            let result: Vec<Value> = vector
                .value
                .into_iter()
                .map(|v| eval(v, environment.clone(), syntax_quote))
                .collect::<Result<Vec<Value>>>()?;

            Value::as_vector(result)
        }
        Value::Map(map) => {
            let result: Vec<(Value, Value)> = map
                .value
                .into_iter()
                .map(|(k, v)| {
                    { eval(k, environment.clone(), syntax_quote) }
                        .and_then(|ek| eval(v, environment.clone(), syntax_quote).map(|ev| (ek, ev)))
                })
                .collect::<Result<Vec<(Value, Value)>>>()?;

            Value::as_map(result)
        }
        Value::Set(set) => {
            let result: Vec<Value> = set
                .value
                .into_iter()
                .map(|v| eval(v, environment.clone(), syntax_quote))
                .collect::<Result<Vec<Value>>>()?;

            Value::as_set(result)
        }
        Value::SplicingMacro(_) => Err(Error::Syntax("splicing macro cannot be evaluated".to_string())),
    }
}
