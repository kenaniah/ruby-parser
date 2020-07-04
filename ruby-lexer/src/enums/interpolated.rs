use crate::Token;

#[derive(Debug, PartialEq)]
pub(crate) enum Interpolated {
    Char(char),
    String(String),
    Expression(Token),
}
