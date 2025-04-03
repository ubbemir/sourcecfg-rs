mod config;
mod statement;
mod param;

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::{Result, Error};

#[derive(Parser)]
#[grammar = "source_cfg.pest"]
pub struct CFGParser;

pub fn parse(input: &str) -> Result<Pair<'_, Rule>> {
    let file = CFGParser::parse(Rule::config, input)
        .map_err(|e| Error::ParsingError(e.to_string()))?
        .next();

    match file {
        Some(e) => Ok(e),
        None => Err(Error::ParsingError("Unknown error".to_string())),
    }
}


pub trait Parseable {
    fn parse(rule: Pair<'_, Rule>) -> Self;
}