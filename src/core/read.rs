use std::collections::VecDeque;

use pest::iterators::Pair;

use crate::core::environment::Environment;
use crate::core::parse::Rule;
use crate::core::value::Value;

pub fn read(environment: &mut Environment, parsed: Pair<Rule>) -> Result<VecDeque<Value>, String> {
    println!("{:#?}", parsed);
    let values: VecDeque<Value>;
    match parsed.as_rule() {
        Rule::scilisp => {
            values = parsed.into_inner().map(|pair| read(environment, pair)).collect();
            println!("{:#?}", values);
            Ok(Value::Nil)
        },
        Rule::nil => Value::as_nil(),
        Rule::bool => Value::as_bool(parsed),
        Rule::i64 => Value::as_i64(parsed),
        Rule::EOI => Ok(Value::Nil),
        _ => unreachable!(),
    }
}

