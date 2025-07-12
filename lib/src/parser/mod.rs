mod config;
mod statement;
mod param;

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

use crate::{Result, Error};
use crate::lang::constructs::Config;

#[derive(Parser)]
#[grammar = "source_cfg.pest"]
pub struct CFGParser;

fn pest_parse(input: &str) -> Result<Pair<'_, Rule>> {
    let file = CFGParser::parse(Rule::config, input)
        .map_err(|e| Error::ParsingError(e.to_string()))?
        .next();

    match file {
        Some(e) => Ok(e),
        None => Err(Error::ParsingError("Unknown error".to_string())),
    }
}

pub fn parse(input: &str) -> Result<Config> {
    let pest_parsed = pest_parse(input)?;

    match Config::parse(pest_parsed) {
        Some(val) => Ok(val),
        None => Err(Error::ParsingError("Failed to parse Config".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let parsed = pest_parse("");

        assert!(parsed.is_ok(), "Expected parsing of empty string to return Ok(_) got {:?}", parsed);
    }
}

pub trait Parseable where Self: Sized {
    fn parse(rule: Pair<'_, Rule>) -> Option<Self>;
}