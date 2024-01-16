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

pub fn splicing_insert(values: Vec<Value>) -> Vec<Value> {
    values
        .into_iter()
        .flat_map(|v| match v {
            Value::Splicing(s) => s.value,
            _ => vec![v],
        })
        .collect()
}

fn eval_rest(rest: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Vec<Value>> {
    let result: Vec<Value> = rest
        .into_iter()
        .map(|v| eval(v, environment))
        .collect::<Result<Vec<Value>>>()?;

    Ok(splicing_insert(result))
}

fn eval_list(list: &List, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
    let list_inner = splicing_insert(list.value.clone());

    let first = match list_inner.first() {
        None => return Ok(Value::List(list.clone())),
        Some(f) => f,
    };

    let mut first: Value = match first {
        Value::Symbol(sym) => environment.borrow().get(sym.clone())?.1.clone(),
        Value::List(list) => eval(Value::List(list.clone()), environment)?,
        Value::Vector(v) => eval(Value::Vector(v.clone()), environment)?,
        f => f.clone(),
    };

    let mut rest: Vec<Value> = list_inner[1..].to_vec();

    // if first of list is splicing, then expand it
    if let Value::Splicing(_) = first.clone() {
        let v = splicing_insert(vec![first]);
        first = v[0].clone();
        let mut r = v[1..].to_vec();
        r.extend(rest.clone());
        rest = r;
    }

    let result: Result<Value> = match first {
        Value::Function(func) => func.borrow_mut().call(eval_rest(rest, environment)?),
        Value::I64(mut int) => int.call(eval_rest(rest, environment)?),
        Value::String(mut s) => s.call(eval_rest(rest, environment)?),
        Value::Keyword(mut k) => k.call(eval_rest(rest, environment)?),
        Value::Vector(v) => v.call(eval_rest(rest, environment)?),
        Value::Macro(mac) => mac.borrow_mut().call(rest, environment), // TODO: splicing for macro rest
        f => Err(Error::Syntax(format!("cannot call '{}'", f))),
    };

    result
}

pub fn eval_ast(ast: &mut Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
    let val = match ast.pop() {
        Some(val) => val,
        None => Value::Nil,
    };
    eval(val, environment)
}

pub fn eval(val: Value, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
    match val {
        Value::Nil
        | Value::Bool(_)
        | Value::I64(_)
        | Value::F64(_)
        | Value::Regex(_)
        | Value::String(_)
        | Value::Keyword(_)
        | Value::Function(_)
        | Value::Macro(_)
        | Value::Generator(_) => Ok(val),
        Value::Splicing(_) => Ok(val), // TODO: remove splicing
        Value::Slice(s) => {
            let start = eval(s.start.clone(), environment)?;
            let end = eval(s.end.clone(), environment)?;
            let step = eval(s.step.clone(), environment)?;
            match (&start, &end, &step) {
                (Value::I64(_), Value::I64(_), Value::I64(_)) => {}
                _ => return Err(Error::Type("slice can contain only i64".to_string())),
            }
            Ok(Value::Slice(Rc::new(Slice::new(start, end, step))))
        }
        // TODO: removed is_need_eval
        Value::Symbol(symbol) => Ok(environment.borrow().get(symbol)?.1.clone()),
        Value::List(list) => eval_list(&list, environment),
        Value::Vector(vector) => {
            let mut result: Vec<Value> = vector
                .value
                .into_iter()
                .map(|v| eval(v, environment))
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
                            _ => eval(k, environment),
                        }
                    }
                    .and_then(|ek| match v {
                        Value::Splicing(_) => Err(Error::Syntax("splicing in map".to_string())),
                        _ => eval(v, environment).map(|ev| (ek, ev)),
                    })
                })
                .collect::<Result<Vec<(Value, Value)>>>()?;

            Value::as_map(result)
        }
        Value::Set(set) => {
            let mut result: Vec<Value> = set
                .value
                .into_iter()
                .map(|v| eval(v, environment))
                .collect::<Result<Vec<Value>>>()?;

            result = splicing_insert(result);

            Value::as_set(result)
        }
    }
}
