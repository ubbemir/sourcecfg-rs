mod error;
pub use error::*;

pub mod formatters;
pub mod lang;

use crate::Error as CrateErr;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "source_cfg.pest"]
pub struct CFGParser;

pub fn parse(input: &str) -> Result<Pair<'_, Rule>, CrateErr> {
    let file = CFGParser::parse(Rule::config, input)
        .map_err(|e| CrateErr::ParsingError(e.to_string()))?
        .next();

    match file {
        Some(e) => Ok(e),
        None => Err(CrateErr::ParsingError("Unknown error".to_string())),
    }
}
