use crate::parser::Rule;
use pest::iterators::Pair;
use crate::lang::constructs::{Config, Statement};
use crate::parser::Parseable;

impl Parseable for Config {
    fn parse(cfg: Pair<'_, Rule>) -> Option<Self> {
        let stmts = match cfg.into_inner().next() {
            Some(val) => val,
            None => return Some(Config { statements: Vec::new() }),
        };
    
        let stmts_res = match stmts.as_rule() {
            Rule::statements => {
                let filtered = stmts.into_inner().filter(|line| matches!(line.as_rule(), Rule::statement));
                filtered.filter_map(|stmt| Statement::parse(stmt)).collect()
            },
            _ => Vec::new()
        };
    
        Some(Config {
            statements: stmts_res
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_parseable_empty_stmts() {
        let parsed = parse("").expect("TEST FAIL: Expected pest parsing to succeed");
        let cfg = Config::parse(parsed);

        assert!(cfg.is_some(), "Expected parsing to return a valid Config struct. Got: {:?}", cfg);

        let cfg = cfg.unwrap();
        assert_eq!(cfg.statements.len(), 0);
    }
}