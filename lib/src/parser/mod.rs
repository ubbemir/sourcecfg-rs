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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let parsed = parse("");

        assert!(parsed.is_ok(), "Expected parsing of empty string to return Ok(_) got {:?}", parsed);
    }
}

pub trait Parseable where Self: Sized {
    fn parse(rule: Pair<'_, Rule>) -> Option<Self>;
}