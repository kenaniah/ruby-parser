use crate::Token;

pub enum Interpolated {
    ExternalCommand(Vec<Token>),
    String(Vec<Token>),
    Symbol(Vec<Token>),
}
