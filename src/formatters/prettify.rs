use itertools::Itertools;

use crate::lang::constructs::Config;
use crate::parser::Parseable;
use crate::{Result, Error};


pub fn prettify(input: &str) -> Result<String> {
    let file = crate::parser::parse(input)?;
    let cfg = match Config::parse(file) {
        Some(e) => e,
        None => return Err(Error::ParsingError("Invalid config".to_string()))
    };

    Ok(Itertools::intersperse(cfg.statements.iter().map(|stmt| format!("{stmt}")), "\n".to_string()).collect())
}