// TODO: add complex, irrational, bigint
pub enum Literal {
    Nil,
    True,
    False,
    Integer(isize),
    Float(f64),
    String(String),
    Symbol(String),
    ExternalCommand(String),
}
