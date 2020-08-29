use super::*;
use crate::lexer::*;

#[derive(Debug, PartialEq)]
pub enum Node {
    LogicalAnd(LogicalAnd),
    LogicalOr(LogicalOr),
    LogicalNot(LogicalNot),
    Literal(Literal),
    Identifier(Identifier),
    Interpolated(Interpolated),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Block(Vec<Node>),
    Segment(Segment),
    Comment(String),
    Nil,
    Self_,
    EndOfProgram,
    Placeholder,
}

#[allow(dead_code)]
impl Node {
    /// Creates a token that represents a boolean value
    pub(crate) fn boolean(val: bool) -> Self {
        Self::Literal(Literal::Boolean(val))
    }
    /// Creates a token that represents an integer value
    pub(crate) fn integer(val: isize) -> Self {
        Self::Literal(Literal::Integer(val))
    }
    /// Creates a token that represents a float value
    pub(crate) fn float(val: f64) -> Self {
        Self::Literal(Literal::Float(val))
    }
    /// Creates a token that represents a literal string
    pub(crate) fn literal_string(val: &str) -> Self {
        Self::Literal(Literal::String(val.to_owned()))
    }
    /// Creates a token that represents a float value
    pub(crate) fn literal_symbol(val: &str) -> Self {
        Self::Literal(Literal::Symbol(val.to_owned()))
    }
    /// Creates a token that represents an identifier
    pub(crate) fn ident(name: &str, kind: IdentifierKind) -> Self {
        Self::Identifier(Identifier {
            name: name.to_owned(),
            kind,
        })
    }
    /// Creates a token that represents a unary operation
    pub(crate) fn unary_op(op: UnaryOpKind, rhs: Node) -> Self {
        Self::UnaryOp(UnaryOp {
            op,
            rhs: Box::new(rhs),
        })
    }
    /// Creates a token that represents a binary operation
    pub(crate) fn binary_op(lhs: Node, op: BinaryOpKind, rhs: Node) -> Self {
        Self::BinaryOp(BinaryOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
    /// Creates a token that represents a logical AND
    pub(crate) fn logical_and(first: Node, second: Node) -> Node {
        Self::LogicalAnd(LogicalAnd {
            first: Box::new(first),
            second: Box::new(second),
        })
    }
    /// Creates a token that represents a logical OR
    pub(crate) fn logical_or(first: Node, second: Node) -> Node {
        Self::LogicalOr(LogicalOr {
            first: Box::new(first),
            second: Box::new(second),
        })
    }
}
