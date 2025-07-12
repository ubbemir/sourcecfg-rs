use itertools::Itertools;

use crate::Result;


pub fn prettify(input: &str) -> Result<String> {
    let cfg = crate::parser::parse(input)?;

    Ok(Itertools::intersperse(cfg.statements.iter().map(|stmt| format!("{stmt}")), "\n".to_string()).collect())
}