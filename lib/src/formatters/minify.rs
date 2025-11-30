use itertools::Itertools;

use crate::{Result, lang::constructs::Config};

pub fn minify(config: &Config) -> Result<String> {
    Ok(Itertools::intersperse(
        config.statements.iter().map(|stmt| format!("{stmt}")),
        ";".to_string(),
    )
    .collect())
}
