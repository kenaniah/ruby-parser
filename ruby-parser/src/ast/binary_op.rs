use super::*;

#[derive(Debug, PartialEq)]
pub struct BinaryOp {
    pub op: BinaryOpKind,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOpKind {
    /// <=>
    Compare,
    /// ==
    Equal,
    /// ===
    CaseEqual,
    /// !=
    NotEqual,
    /// =~
    RegexMatch,
    /// !~
    NotRegexMatch,
    /// >
    GreaterThan,
    /// >=
    GreaterEqual,
    /// <
    LessThan,
    /// <=
    LessEqual,
    /// |
    BitOr,
    /// ^
    BitXor,
    /// &
    BitAnd,
    /// <<
    ShiftLeft,
    /// >>
    ShiftRight,
    /// +
    Add,
    /// -
    Subtract,
    /// *
    Multiply,
    /// /
    Divide,
    /// %
    Modulus,
    /// **
    Power,
}
