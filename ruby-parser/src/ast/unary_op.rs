use super::*;

#[derive(Debug, PartialEq)]
pub struct UnaryOp {
    pub op: UnaryOpKind,
    pub rhs: Box<Node>
}

#[derive(Debug, PartialEq)]
pub enum UnaryOpKind {
    /// ~
    BitNot,
    /// +
    Positive,
    /// -
    Negative,
    /// !
    LogicalNot
}

impl From<char> for UnaryOpKind {
    fn from(c: char) -> Self {
        match c {
            '~' => Self::BitNot,
            '+' => Self::Positive,
            '-' => Self::Negative,
            '!' => Self::LogicalNot,
            _ => unreachable!()
        }
    }
}
