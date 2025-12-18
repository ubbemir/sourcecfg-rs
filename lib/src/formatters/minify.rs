use itertools::Itertools;

use crate::lang::constructs::Config;

pub fn minify(config: &Config) -> String {
    Itertools::intersperse(
        config.statements.iter().map(|stmt| format!("{stmt}")),
        ";".to_string(),
    )
    .collect()
}
