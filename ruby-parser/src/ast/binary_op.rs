use super::*;

pub struct BinaryOp {
    pub op: BinaryOpToken,
    pub lhs: Expr,
    pub rhs: Expr
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
