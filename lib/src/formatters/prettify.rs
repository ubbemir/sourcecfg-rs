use itertools::Itertools;

use crate::{Result, lang::constructs::Config};

pub fn prettify(config: &Config) -> Result<String> {
    Ok(Itertools::intersperse(
        config.statements.iter().map(|stmt| format!("{stmt}")),
        "\n".to_string(),
    )
    .collect())
}
