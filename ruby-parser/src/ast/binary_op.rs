use super::*;

pub struct BinaryOp {
    pub op: BinaryOpToken,
    pub lhs: Box<Node>,
    pub rhs: Box<Node>
}

pub enum BinaryOpToken {
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
    Power
}
