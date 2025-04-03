use itertools::Itertools;

use crate::lang::constructs::parse_config;
use crate::Error;


pub fn prettify(input: &str) -> Result<String, Error> {
    let file = crate::parse(input)?;
    let cfg = parse_config(file);

    Ok(Itertools::intersperse(cfg.statements.iter().map(|stmt| format!("{stmt}")), "\n".to_string()).collect())
}