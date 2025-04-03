use crate::parser::Rule;
use pest::iterators::Pair;
use crate::lang::constructs::{Statement, Param};
use crate::parser::Parseable;

impl Parseable for Statement {
    fn parse(stmt: Pair<'_, Rule>) -> Statement {
        let mut cvar = String::new();
        let mut params_res: Vec<Param> = Vec::new();
    
        let params = stmt.into_inner();
        for param in params {
            match param.as_rule() {
                Rule::cvar => {
                    cvar = param.as_str().to_string();
                }
                Rule::param => {
                    params_res.push(Param::parse(param.into_inner().next().unwrap()));
                }
                _ => (),
            }
        }
    
        Statement {
            cvar: cvar,
            params: params_res,
        }
    }
}