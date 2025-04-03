use core::fmt;

use crate::Rule;
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Config {
    pub statements: Vec<Statement>
}

pub fn parse_config(cfg: Pair<'_, Rule>) -> Config {
    let stmts = match cfg.into_inner().next() {
        Some(stmts) => stmts,
        None => return Config { statements: Vec::new() }
    };

    let stmts_res = match stmts.as_rule() {
        Rule::statements => {
            let filtered = stmts.into_inner().filter(|line| if let Rule::statement = line.as_rule() { true } else { false });
            filtered.map(|stmt| parse_statement(stmt)).collect()
        },
        _ => Vec::new()
    };

    Config {
        statements: stmts_res
    }
}

#[derive(Debug)]
pub struct Statement {
    pub cvar: String,
    pub params: Vec<Param>,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cvar)?;
        for param in self.params.iter() {
            write!(f, " {}", param)?;
        }
        Ok(())
    }
}

pub fn parse_statement(stmt: Pair<'_, Rule>) -> Statement {
    let mut cvar = String::new();
    let mut params_res: Vec<Param> = Vec::new();

    let params = stmt.into_inner();
    for param in params {
        match param.as_rule() {
            Rule::cvar => {
                cvar = param.as_str().to_string();
            }
            Rule::param => {
                params_res.push(parse_param(param.into_inner().next().unwrap()));
            }
            _ => (),
        }
    }

    Statement {
        cvar: cvar,
        params: params_res,
    }
}

#[derive(Debug)]
pub enum Param {
    Bool(bool),
    Float(f64),
    Int(i64),
    Cvar(String),
    String(String),
}
impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Param::Bool(b) => write!(f, "{}", b),
            Param::Float(n) => write!(f, "{}", n),
            Param::Int(n) => write!(f, "{}", n),
            Param::Cvar(s) | Param::String(s) => write!(f, "{}", s),
        }
    }
}

pub fn parse_param(p: Pair<'_, Rule>) -> Param {
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
