use crate::Token;

/// Defines a segment of something that may be interpolated
#[derive(Debug, PartialEq)]
pub(crate) enum Segment {
    Char(char),
    String(String),
    Expr(Token),
}
