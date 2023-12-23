/* read.rs */

use pest::iterators::Pair;

use crate::core::parse::Rule;
use crate::core::value::Value;

fn inner_collect(ast: &mut Vec<Value>, pair: Pair<Rule>) -> Result<Vec<Value>, String> {
    pair.into_inner().map(|expr| read(ast, expr)).collect()
}

pub fn read(ast: &mut Vec<Value>, pair: Pair<Rule>) -> Result<Value, String> {
    let value = match pair.as_rule() {
        Rule::scilisp => read(ast, pair.into_inner().next().unwrap()),
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
            let pairs = pair.into_inner().next().unwrap().into_inner().collect::<Vec<_>>();
            let result = pairs.chunks_exact(2).map(|p| {
                (read(ast, p[0].clone()).unwrap(), read(ast, p[1].clone()).unwrap())
            }).collect();
            result
        }),
        Rule::set => Value::as_set(inner_collect(ast, pair)?),
        Rule::special_form => Value::as_special_form(pair),
        _ => unreachable!(), // COMMENT, WHITESPACE, etc...
    };

    match value {
        Ok(value) => {
            ast.push(value.clone());
            Ok(value)
        }
        Err(err) => Err(err),
    }
}
