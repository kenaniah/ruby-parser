// TODO: add complex, irrational, bigint
#[derive(Debug, PartialEq)]
pub enum Literal {
    Nil,
    Boolean(bool),
    Integer(isize),
    Float(f64),
    String(String),
    Symbol(String),
    ExternalCommand(String),
}
