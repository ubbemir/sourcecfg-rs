use crate::parser::Rule;
use pest::iterators::Pair;
use crate::lang::constructs::Param;
use crate::parser::Parseable;

impl Parseable for Param {
    fn parse(p: Pair<'_, Rule>) -> Option<Self> {
        let p_inner = match p.into_inner().next() {
            Some(e) => e,
            None => return None
        };

        Some(match p_inner.as_rule() {
            Rule::bool => {
                let inner = match p_inner.as_str() {
                    "true" => true,
                    _ => false,
                };
    
                Param::Bool(inner)
            },
            Rule::number => {
                let num_str = p_inner.as_str();
                if let Ok(i) = num_str.parse::<i64>() {
                    Param::Int(i)
                } else if let Ok(f) = num_str.parse::<f64>() {
                    Param::Float(f)
                } else {
                    Param::String(num_str.to_string())
                }
            },
            Rule::cvar => Param::Cvar(p_inner.as_str().to_string()),
            _ => Param::String(p_inner.as_str().to_string()),
        })
    }
}