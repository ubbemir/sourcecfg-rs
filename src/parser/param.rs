use crate::parser::Rule;
use pest::iterators::Pair;
use crate::lang::constructs::Param;
use crate::parser::Parseable;

impl Parseable for Param {
    fn parse(p: Pair<'_, Rule>) -> Param {
        match p.as_rule() {
            Rule::bool => {
                let inner = match p.as_str() {
                    "true" => true,
                    _ => false,
                };
    
                Param::Bool(inner)
            },
            Rule::number => {
                let num_str = p.as_str();
                if let Ok(i) = num_str.parse::<i64>() {
                    Param::Int(i)
                } else if let Ok(f) = num_str.parse::<f64>() {
                    Param::Float(f)
                } else {
                    Param::String(num_str.to_string())
                }
            },
            Rule::cvar => Param::Cvar(p.as_str().to_string()),
            _ => Param::String(p.as_str().to_string()),
        }
    }
}