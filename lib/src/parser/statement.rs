use crate::lang::constructs::{Param, Statement};
use crate::parser::Parseable;
use crate::parser::Rule;
use pest::iterators::Pair;

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
                    if let Some(p) = p {
                        params_res.push(p)
                    }
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
