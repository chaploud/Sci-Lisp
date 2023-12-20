use pest::{Parser, iterators::Pairs};
use pest_derive::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "core/pest/grammar.pest"]
pub struct SciLispParser;

pub fn parse(input: &str) -> Result<Pairs<'_, Rule>, Error<Rule>>{
    SciLispParser::parse(Rule::scilisp, input)
}
