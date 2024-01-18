/* core/read.rs */

use std::rc::Rc;

use pest::iterators::Pair;

use crate::core::builtin::macros::*;
use crate::core::parse::Rule;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::types::slice::Slice;
use crate::core::value::Value;

fn inner_collect(pair: Pair<Rule>) -> Result<Vec<Value>> {
    pair.into_inner().map(|expr| read_scilisp(expr)).collect()
}

fn read_scilisp(pair: Pair<Rule>) -> Result<Value> {
    let value = match pair.as_rule() {
        Rule::nil => Value::as_nil(),
        Rule::bool => Value::as_bool(pair),
        Rule::i64 => Value::as_i64(pair),
        Rule::f64 => Value::as_f64(pair),
        Rule::symbol => Value::as_symbol(pair),
        Rule::keyword => Value::as_keyword(pair),
        Rule::regex => Value::as_regex(pair),
        Rule::string => Value::as_string(pair),
        Rule::list => Value::as_list(inner_collect(pair)?),
        Rule::vector => Value::as_vector(inner_collect(pair)?),
        Rule::map => Value::as_map({
            let pairs = pair.into_inner().collect::<Vec<_>>();
            let result: Result<Vec<(Value, Value)>> = pairs
                .chunks(2)
                .map(|p| {
                    {
                        if p.len() != 2 {
                            return Err(Error::Syntax(
                                "map must have even number of elements".to_string(),
                            ));
                        }
                        let key = match p[0].as_rule() {
                            Rule::symbol
                            | Rule::keyword
                            | Rule::string
                            | Rule::i64
                            | Rule::list => p[0].clone(),
                            _ => {
                                return Err(Error::Syntax(
                                    "map keys must be keyword, string or i64 after evaluated"
                                        .to_string(),
                                ))
                            }
                        };
                        read_scilisp(key)
                    }
                    .and_then(|key| read_scilisp(p[1].clone()).map(|value| (key, value)))
                })
                .collect();
            result?
        }),
        Rule::set => Value::as_set(inner_collect(pair)?),
        Rule::quote => quote_to_ast(pair),
        Rule::syntax_quote => syntax_quote_to_ast(pair),
        Rule::unquote => unquote_to_ast(pair),
        Rule::unquote_splicing => unquote_splicing_to_ast(pair),
        Rule::slice => as_slice(pair),
        _ => {
            println!("pair: {:?}", pair.as_str());
            Err(Error::Syntax("unexpected token".to_string()))
        }
    };

    value
}

pub fn read(ast: &mut Vec<Value>, pair: Pair<Rule>) -> Result<()> {
    let toplevel = pair.into_inner().next().unwrap().into_inner(); // scilisp->scilisp_inner
    for expr in toplevel {
        if expr.as_rule() != Rule::EOI {
            ast.push(read_scilisp(expr)?);
        }
    }
    Ok(())
}

fn quote_to_ast(pair: Pair<Rule>) -> Result<Value> {
    let pair = pair.into_inner().next().unwrap();
    let value = read_scilisp(pair)?;
    Value::as_list(vec![Value::Symbol((*SYMBOL_QUOTE).clone()), value])
}

fn syntax_quote_to_ast(pair: Pair<Rule>) -> Result<Value> {
    let pair = pair.into_inner().next().unwrap();
    let value = read_scilisp(pair)?;
    Value::as_list(vec![Value::Symbol((*SYMBOL_SYNTAX_QUOTE).clone()), value])
}

fn unquote_to_ast(pair: Pair<Rule>) -> Result<Value> {
    let pair = pair.into_inner().next().unwrap();
    let value = read_scilisp(pair)?;
    Value::as_list(vec![Value::Symbol((*SYMBOL_UNQUOTE).clone()), value])
}

fn unquote_splicing_to_ast(pair: Pair<Rule>) -> Result<Value> {
    let pair = pair.into_inner().next().unwrap();
    let value = read_scilisp(pair)?;
    Value::as_list(vec![
        Value::Symbol((*SYMBOL_UNQUOTE_SPLICING).clone()),
        value,
    ])
}

fn as_slice(pair: Pair<Rule>) -> Result<Value> {
    let mut slice_start = Value::Nil;
    let mut slice_end = Value::Nil;
    let mut slice_step = Value::Nil;

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::slice_start => slice_start = read_scilisp(p.into_inner().next().unwrap())?,
            Rule::slice_end => slice_end = read_scilisp(p.into_inner().next().unwrap())?,
            Rule::slice_step => slice_step = read_scilisp(p.into_inner().next().unwrap())?,
            _ => unreachable!(),
        }
    }

    Ok(Value::Slice(Rc::new(Slice::new(
        slice_start,
        slice_end,
        slice_step,
    ))))
}
