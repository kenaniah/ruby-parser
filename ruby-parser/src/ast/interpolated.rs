use crate::lexer::Token;

pub enum Interpolated {
    Command(Vec<Token>),
    String(Vec<Token>),
    Symbol(Vec<Token>),
}
