use itertools::Itertools;

use crate::lang::constructs::Config;
use crate::parser::Parseable;
use crate::Result;


pub fn minify(input: &str) -> Result<String> {
    let file = crate::parser::parse(input)?;
    let cfg = Config::parse(file);

    Ok(Itertools::intersperse(cfg.statements.iter().map(|stmt| format!("{stmt}")), ";".to_string()).collect())
}