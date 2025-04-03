use crate::parser::Rule;
use pest::iterators::Pair;
use crate::lang::constructs::{Config, Statement};
use crate::parser::Parseable;

impl Parseable for Config {
    fn parse(cfg: Pair<'_, Rule>) -> Option<Self> {
        let stmts = match cfg.into_inner().next() {
            Some(stmts) => stmts,
            None => return None
        };
    
        let stmts_res = match stmts.as_rule() {
            Rule::statements => {
                let filtered = stmts.into_inner().filter(|line| if let Rule::statement = line.as_rule() { true } else { false });
                filtered.map(|stmt| Statement::parse(stmt)).flatten().collect() // ".flatten" removes Option::None types
            },
            _ => Vec::new()
        };
    
        Some(Config {
            statements: stmts_res
        })
    }
}