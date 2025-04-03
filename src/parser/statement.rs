use crate::parser::Rule;
use pest::iterators::Pair;
use crate::lang::constructs::{Statement, Param};
use crate::parser::Parseable;

impl Parseable for Statement {
    fn parse(stmt: Pair<'_, Rule>) -> Option<Self> {
        let mut cvar = String::new();
        let mut params_res: Vec<Param> = Vec::new();
    
        let params = stmt.into_inner();
        for param in params {
            match param.as_rule() {
                Rule::cvar => {
                    cvar = param.as_str().to_string();
                }
                Rule::param => {
                    let p = Param::parse(param);
                    if let Some(p) = p { params_res.push(p) }
                }
                _ => (),
            }
        }
    
        Some(Statement {
            cvar,
            params: params_res,
        })
    }
}