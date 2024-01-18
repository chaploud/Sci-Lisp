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

fn eval_rest(rest: Vec<Value>, environment: &Rc<RefCell<Environment>>) -> Result<Vec<Value>> {
    let result: Vec<Value> = rest
        .into_iter()
        .map(|v| eval(v, environment))
        .collect::<Result<Vec<Value>>>()?;

    Ok(result)
}

fn eval_list(list: &List, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
    let list_inner = list.value.clone();

    let first = match list_inner.first() {
        None => return Ok(Value::List(list.clone())),
        Some(f) => f,
    };

    let first: Value = match first {
        Value::Symbol(sym) => environment.borrow().get(sym)?.1.clone(),
        Value::List(list) => eval(Value::List(list.clone()), environment)?,
        Value::Vector(v) => eval(Value::Vector(v.clone()), environment)?,
        f => f.clone(),
    };

    let rest: Vec<Value> = list_inner[1..].to_vec();

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

pub fn eval(value: Value, environment: &Rc<RefCell<Environment>>) -> Result<Value> {
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
        | Value::ControlFlowMacro(_)
        | Value::ControlFlow(_)
        | Value::Generator(_) => Ok(value),
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
        Value::Symbol(symbol) => Ok(environment.borrow().get(&symbol)?.1.clone()),
        Value::List(list) => eval_list(&list, environment),
        Value::Vector(vector) => {
            let result: Vec<Value> = vector
                .value
                .into_iter()
                .map(|v| eval(v, environment))
                .collect::<Result<Vec<Value>>>()?;

            Value::as_vector(result)
        }
        Value::Map(map) => {
            let result: Vec<(Value, Value)> = map
                .value
                .into_iter()
                .map(|(k, v)| {
                    { eval(k, environment) }.and_then(|ek| eval(v, environment).map(|ev| (ek, ev)))
                })
                .collect::<Result<Vec<(Value, Value)>>>()?;

            Value::as_map(result)
        }
        Value::Set(set) => {
            let result: Vec<Value> = set
                .value
                .into_iter()
                .map(|v| eval(v, environment))
                .collect::<Result<Vec<Value>>>()?;

            Value::as_set(result)
        }
        Value::SplicingMacro(_) => Err(Error::Syntax(
            "splicing macro cannot be evaluated".to_string(),
        )),
    }
}
