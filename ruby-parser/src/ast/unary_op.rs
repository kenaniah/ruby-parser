use super::*;

pub struct UnaryOp {
    pub op: UnaryOpToken,
    pub rhs: Box<Node>
}

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
