use crate::Token;

pub(crate) enum Interpolated {
    Char(char),
    String(String),
    Expression(Token),
}
