/* core/read.rs */

use pest::iterators::Pair;

use crate::core::builtin::macros::{
    SYMBOL_QUOTE, SYMBOL_SYNTAX_QUOTE, SYMBOL_UNQUOTE, SYMBOL_UNQUOTE_SPLICING,
};
use crate::core::parse::Rule;
use crate::core::types::error::Error;
use crate::core::types::error::Result;
use crate::core::value::Value;

fn inner_collect(ast: &mut Vec<Value>, pair: Pair<Rule>) -> Result<Vec<Value>> {
    pair.into_inner()
        .map(|expr| read_scilisp(ast, expr))
        .collect()
}

fn read_scilisp(ast: &mut Vec<Value>, pair: Pair<Rule>) -> Result<Value> {
    let value = match pair.as_rule() {
        Rule::scilisp => read_scilisp(ast, pair.into_inner().next().unwrap()),
        Rule::nil => Value::as_nil(),
        Rule::bool => Value::as_bool(pair),
        Rule::i64 => Value::as_i64(pair),
        Rule::f64 => Value::as_f64(pair),
        Rule::symbol => Value::as_symbol(pair),
        Rule::keyword => Value::as_keyword(pair),
        Rule::regex => Value::as_regex(pair),
        Rule::string => Value::as_string(pair),
        Rule::list => Value::as_list(inner_collect(ast, pair)?),
        Rule::vector => Value::as_vector(inner_collect(ast, pair)?),
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
                            } // TODO: check in eval
                        };
                        read_scilisp(ast, key)
                    }
                    .and_then(|key| read_scilisp(ast, p[1].clone()).map(|value| (key, value)))
                })
                .collect();
            result?
        }),
        Rule::set => Value::as_set(inner_collect(ast, pair)?),
        Rule::quote => quote_to_ast(ast, pair),
        Rule::syntax_quote => syntax_quote_to_ast(ast, pair),
        Rule::unquote => unquote_to_ast(ast, pair),
        Rule::unquote_splicing => unquote_splicing_to_ast(ast, pair),
        _ => {
            println!("pair: {:?}", pair.as_str());
            Err(Error::Syntax("unexpected token".to_string()))
        }
    };

    match value {
        Ok(value) => {
            ast.push(value.clone());
            Ok(value)
        }
        Err(err) => Err(err),
    }
}

pub fn read(ast: &mut Vec<Value>, pair: Pair<Rule>) -> Result<()> {
    if pair.as_rule() != Rule::scilisp {
        return Err(Error::Syntax("expected scilisp expression".to_string()));
    }
    for expr in pair.into_inner() {
        if expr.as_rule() != Rule::EOI {
            read_scilisp(ast, expr)?;
        }
    }
    Ok(())
}

fn quote_to_ast(ast: &mut Vec<Value>, pair: Pair<Rule>) -> Result<Value> {
    let pair = pair.into_inner().next().unwrap();
    let value = read_scilisp(ast, pair)?;
    Value::as_list(vec![Value::Symbol(SYMBOL_QUOTE), value])
}

fn syntax_quote_to_ast(ast: &mut Vec<Value>, pair: Pair<Rule>) -> Result<Value> {
    let pair = pair.into_inner().next().unwrap();
    let value = read_scilisp(ast, pair)?;
    Value::as_list(vec![Value::Symbol(SYMBOL_SYNTAX_QUOTE), value])
}

fn unquote_to_ast(ast: &mut Vec<Value>, pair: Pair<Rule>) -> Result<Value> {
    let pair = pair.into_inner().next().unwrap();
    let value = read_scilisp(ast, pair)?;
    Value::as_list(vec![Value::Symbol(SYMBOL_UNQUOTE), value])
}

fn unquote_splicing_to_ast(ast: &mut Vec<Value>, pair: Pair<Rule>) -> Result<Value> {
    let pair = pair.into_inner().next().unwrap();
    let value = read_scilisp(ast, pair)?;
    Value::as_list(vec![Value::Symbol(SYMBOL_UNQUOTE_SPLICING), value])
}
