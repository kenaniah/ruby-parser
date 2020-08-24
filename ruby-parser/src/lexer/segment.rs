use super::Token;

/// Defines a segment of something that may be interpolated
#[derive(Debug, PartialEq)]
pub enum Segment {
    Char(char),
    String(String),
    Expr(Token),
}
