use pest::iterators::Pair;

use crate::core::environment::Environment;
use crate::core::parse::Rule;
use crate::core::value::Value;

pub fn read(environment: &mut Environment, parsed: Pair<Rule>) -> Result<Value, String> {
    match parsed.as_rule() {
        Rule::nil => Value::as_nil(),
        Rule::bool => Value::as_bool(parsed),
        Rule::i64 => Value::as_i64(parsed),
        _ => unreachable!(),
    }
}

