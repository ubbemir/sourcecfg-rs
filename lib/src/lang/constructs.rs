use core::fmt;

#[derive(Debug)]
pub struct Config {
    pub statements: Vec<Statement>,
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
