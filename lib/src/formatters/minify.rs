use itertools::Itertools;

use crate::Result;


pub fn minify(input: &str) -> Result<String> {
    let cfg = crate::parser::parse(input)?;

    Ok(Itertools::intersperse(cfg.statements.iter().map(|stmt| format!("{stmt}")), ";".to_string()).collect())
}