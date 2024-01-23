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

fn splicing_expand(values: Vec<Value>) -> Vec<Value> {
    let mut result = Vec::<Value>::new();
    for value in values {
        match value {
            Value::Splicing(splicing) => {
                for v in splicing {
                    result.push(v);
                }
            }
            v => result.push(v),
        }
    }
    result
}

fn eval_rest(rest: Vec<Value>, environment: Rc<RefCell<Environment>>) -> Result<Vec<Value>> {
    let result: Vec<Value> = rest
        .into_iter()
        .map(|v| eval(v, environment.clone(), false))
        .collect::<Result<Vec<Value>>>()?;

    Ok(result)
}

fn eval_list(list: List, environment: Rc<RefCell<Environment>>, syntax_quote: bool) -> Result<Value> {
    let mut list_inner = list.value.clone();
    list_inner = splicing_expand(list_inner);

    let first = match list_inner.first() {
        None => return Ok(Value::List(list.clone())),
        Some(f) => f,
    };

    let unquote = match first {
        Value::Symbol(sym) => *sym == *SYMBOL_UNQUOTE || *sym == *SYMBOL_UNQUOTE_SPLICING,
        _ => false,
    };

    if syntax_quote && !unquote {
        let mut result: Vec<Value> = list_inner
            .into_iter()
            .map(|v| eval(v, environment.clone(), syntax_quote))
            .collect::<Result<Vec<Value>>>()?;

        result = splicing_expand(result);

        return Value::as_list(result);
    }

    let mut first: Value = match first {
        Value::Symbol(sym) => environment.borrow().get(sym)?,
        Value::List(list) => eval(Value::List(list.clone()), environment.clone(), syntax_quote)?,
        Value::Vector(v) => eval(Value::Vector(v.clone()), environment.clone(), syntax_quote)?,
        f => f.clone(),
    };

    let mut rest: Vec<Value> = list_inner[1..].to_vec();
    if let Value::Splicing(s) = first.clone() {
        rest = s[1..].iter().cloned().chain(rest).collect();
        first = s[0].clone();
    }

    let result: Result<Value> = match first {
        Value::Function(func) => {
            rest = eval_rest(rest, environment)?;
            rest = splicing_expand(rest);
            func.call(rest)
        }
        Value::I64(int) => {
            rest = eval_rest(rest, environment)?;
            rest = splicing_expand(rest);
            int.call(rest)
        }
        Value::String(s) => {
            rest = eval_rest(rest, environment)?;
            rest = splicing_expand(rest);
            s.call(rest)
        }
        Value::Keyword(k) => {
            rest = eval_rest(rest, environment)?;
            rest = splicing_expand(rest);
            k.call(rest)
        }
        Value::Vector(v) => {
            rest = eval_rest(rest, environment)?;
            rest = splicing_expand(rest);
            v.call(rest)
        }
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
        | Value::Generator(_)
        | Value::Splicing(_) => Ok(value),
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
            let mut result: Vec<Value> = vector
                .value
                .into_iter()
                .map(|v| eval(v, environment.clone(), syntax_quote))
                .collect::<Result<Vec<Value>>>()?;

            result = splicing_expand(result);

            Value::as_vector(result)
        }
        Value::Map(map) => {
            let result: Vec<(Value, Value)> = map
                .value
                .into_iter()
                .map(|(k, v)| {
                    let ek = eval(k, environment.clone(), syntax_quote)?;
                    let ev = eval(v, environment.clone(), syntax_quote)?;
                    if let Value::Splicing(_) = ek {
                        return Err(Error::Syntax("cannot splice in map key".to_string()));
                    }
                    if let Value::Splicing(_) = ev {
                        return Err(Error::Syntax("cannot splice in map value".to_string()));
                    }
                    Ok((ek, ev))
                })
                .collect::<Result<Vec<(Value, Value)>>>()?;

            Value::as_map(result)
        }
        Value::Set(set) => {
            let mut result: Vec<Value> = set
                .value
                .into_iter()
                .map(|v| eval(v, environment.clone(), syntax_quote))
                .collect::<Result<Vec<Value>>>()?;

            result = splicing_expand(result);

            Value::as_set(result)
        }
    }
}
