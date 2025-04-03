use itertools::Itertools;

use crate::lang::constructs::parse_config;
use crate::Result;


pub fn minify(input: &str) -> Result<String> {
    let file = crate::parser::parse(input)?;
    let cfg = parse_config(file);

    Ok(Itertools::intersperse(cfg.statements.iter().map(|stmt| format!("{stmt}")), ";".to_string()).collect())
}