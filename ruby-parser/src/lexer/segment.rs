use super::*;

/// Defines a segment of something that may be interpolated
#[derive(Debug, PartialEq)]
pub enum Segment {
    Char(char),
    String(String),
    Expr(Box<Node>),
}

impl Segment {
    pub fn expr(v: Node) -> Self {
        Self::Expr(Box::new(v))
    }
}
