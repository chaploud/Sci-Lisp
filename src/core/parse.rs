/* core/parse.rs */

use pest::iterators::Pair;
use pest::Parser as ParserTrait;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "core/pest/grammar.pest"]
pub struct Parser;

pub fn parse(input: &str) -> Result<Pair<Rule>, String> {
    let result = Parser::parse(Rule::scilisp, input);
    match result {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();
            Ok(pair)
        }
        Err(err) => Err(err.to_string()),
    }
}
