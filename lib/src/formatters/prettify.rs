use itertools::Itertools;

use crate::lang::constructs::Config;

pub fn prettify(config: &Config) -> String {
    Itertools::intersperse(
        config.statements.iter().map(|stmt| format!("{stmt}")),
        "\n".to_string(),
    )
    .collect()
}
