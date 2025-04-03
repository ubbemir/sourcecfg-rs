use crate::parser::Rule;
use pest::iterators::Pair;
use crate::lang::constructs::{Config, Statement};
use crate::parser::Parseable;

impl Parseable for Config {
    fn parse(cfg: Pair<'_, Rule>) -> Config {
        let stmts = match cfg.into_inner().next() {
            Some(stmts) => stmts,
            None => return Config { statements: Vec::new() }
        };
    
        let stmts_res = match stmts.as_rule() {
            Rule::statements => {
                let filtered = stmts.into_inner().filter(|line| if let Rule::statement = line.as_rule() { true } else { false });
                filtered.map(|stmt| Statement::parse(stmt)).collect()
            },
            _ => Vec::new()
        };
    
        Config {
            statements: stmts_res
        }
    }
}