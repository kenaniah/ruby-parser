use super::*;

#[derive(Debug, PartialEq)]
pub struct UnaryOp {
    pub op: UnaryOpToken,
    pub rhs: Box<Node>
}

#[derive(Debug, PartialEq)]
pub enum UnaryOpToken {
    /// ~
    BitNot,
    /// +
    Positive,
    /// -
    Negative,
    /// !
    LogicalNot
}

impl From<char> for UnaryOpToken {
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
