pub struct Statement {
    cvar: String,
    params: Vec<Param>
}

pub enum Param {
    Bool(bool),
    Float(f64),
    Int(i64),
    Cvar(String),
    String(String)
}