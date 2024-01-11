/* core/parse.rs */

use pest::iterators::Pair;
use pest::Parser as ParserTrait;
use pest_derive::Parser;

use crate::core::types::error::Error;
use crate::core::types::error::Result;

#[derive(Parser)]
#[grammar = "core/pest/grammar.pest"]
pub struct Parser;

pub fn parse(input: &str) -> Result<Pair<Rule>> {
    let result = Parser::parse(Rule::scilisp, input);
    match result {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();
            Ok(pair)
        }
        Err(err) => Err(Error::PestParse(Box::new(err))),
    }
}
