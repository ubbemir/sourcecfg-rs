use crate::parser::Rule;
use pest::iterators::Pair;
use crate::lang::constructs::{Config, Statement};
use crate::parser::Parseable;

impl Parseable for Config {
    fn parse(cfg: Pair<'_, Rule>) -> Option<Self> {
        let stmts = cfg.into_inner().next()?;
    
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