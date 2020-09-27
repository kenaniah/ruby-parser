use super::*;
use crate::lexer::*;

#[derive(Debug, PartialEq)]
pub enum Node {
    None,
    Conditional(Conditional),
    LogicalAnd(LogicalAnd),
    LogicalOr(LogicalOr),
    LogicalNot(LogicalNot),
    Literal(Literal),
    Identifier(Identifier),
    Interpolated(Interpolated),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Block(Vec<Self>),
    BlockArg(Box<Self>),
    Segment(Segment),
    Comment(String),
    Ranged(Ranged),
    Defined(Box<Self>),
    Splat(Box<Self>),
    Array(Vec<Self>),
    Hash(Vec<Self>),
    Alias(Alias),
    Undef(Undef),
    Loop(Loop),
    Rescue(Rescue),
    Case(Case),
    Nil,
    Self_,
    Redo,
    Retry,
    Return(Vec<Self>),
    Break(Vec<Self>),
    Next(Vec<Self>),
    EndOfProgram,
    Placeholder,
}

impl From<Identifier> for Node {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

#[allow(dead_code)]
impl Node {
    /// Creates a token that represents an empty block
    pub(crate) fn empty() -> Self {
        Self::Block(vec![])
    }
    /// Creates a token that represents a boolean value
    pub(crate) fn boolean(val: bool) -> Self {
        Self::Literal(Literal::Boolean(val))
    }
    /// Creates a token that represents an integer value
    pub(crate) fn int(val: isize) -> Self {
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
    pub(crate) fn unary_op(op: UnaryOpKind, rhs: Self) -> Self {
        Self::UnaryOp(UnaryOp {
            op,
            rhs: Box::new(rhs),
        })
    }
    /// Creates a token that represents a binary operation
    pub(crate) fn binary_op(lhs: Self, op: BinaryOpKind, rhs: Self) -> Self {
        Self::BinaryOp(BinaryOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
    /// Creates a token that represents a logical AND
    pub(crate) fn logical_and(first: Self, second: Self) -> Self {
        Self::LogicalAnd(LogicalAnd {
            first: Box::new(first),
            second: Box::new(second),
        })
    }
    /// Creates a token that represents a logical OR
    pub(crate) fn logical_or(first: Self, second: Self) -> Self {
        Self::LogicalOr(LogicalOr {
            first: Box::new(first),
            second: Box::new(second),
        })
    }
    /// Creates a token that represents a logical NOT
    pub(crate) fn logical_not(expr: Self) -> Self {
        Self::LogicalNot(LogicalNot {
            expr: Box::new(expr),
        })
    }
    /// Creates a token that reprents a defined? statement
    pub(crate) fn defined(node: Self) -> Self {
        Self::Defined(Box::new(node))
    }
    /// Creates a token that reprents a splat argument
    pub(crate) fn splat(node: Self) -> Self {
        Self::Splat(Box::new(node))
    }
    /// Creates a token that reprents a splat argument
    pub(crate) fn block_arg(node: Self) -> Self {
        Self::BlockArg(Box::new(node))
    }
    /// Creates a token that reprents an array constructor
    pub(crate) fn array(node: Vec<Self>) -> Self {
        Self::Array(node)
    }
    /// Creates a token that reprents a hash constructor
    pub(crate) fn hash(node: Vec<Self>) -> Self {
        Self::Hash(node)
    }
    /// Creates a token that reprents a range
    pub(crate) fn range(from: Self, to: Self, exclusive: bool) -> Self {
        Self::Ranged(Ranged {
            from: Box::new(from),
            to: Box::new(to),
            exclusive,
        })
    }
    /// Creates a token that represents a case statement
    pub(crate) fn case(expr: Self, when: Vec<WhenClause>, otherwise: Self) -> Self {
        Self::Case(Case {
            expr: Box::new(expr),
            when: when,
            otherwise: Box::new(otherwise),
        })
    }
    /// Creates a token that represents a conditional statement
    pub(crate) fn conditional(
        kind: ConditionalKind,
        cond: Self,
        then: Self,
        otherwise: Self,
    ) -> Self {
        Self::Conditional(Conditional {
            kind,
            cond: Box::new(cond),
            then: Box::new(then),
            otherwise: Box::new(otherwise),
        })
    }
    /// Creates a token that represents a loop expression
    pub(crate) fn loop_(kind: LoopKind, cond: Self, body: Self, bindings: Vec<Self>) -> Self {
        Self::Loop(Loop {
            kind,
            cond: Box::new(cond),
            body: Box::new(body),
            bindings: if bindings.len() > 0 {
                Some(bindings)
            } else {
                None
            },
        })
    }
    /// Creates a token that represents a rescued statement
    pub(crate) fn rescued_statement(body: Self, rescued: Self) -> Self {
        Self::Rescue(Rescue {
            body: Box::new(body),
            rescue: vec![RescueClause {
                exceptions: vec![],
                assigned_to: Box::new(Self::None),
                then: Box::new(rescued),
            }],
            otherwise: Box::new(Self::None),
        })
    }
    /// Creates a token that represents an alias
    pub(crate) fn alias(to: Identifier, from: Identifier) -> Self {
        Self::Alias(Alias { to, from })
    }
    /// Creates a token that represents an undefinition
    pub(crate) fn undef(list: Vec<Identifier>) -> Self {
        Self::Undef(Undef { list })
    }
    /// Allows placeholding nodes to be updated when working around left-recursion via LL(2)
    pub(crate) fn update_placeholder(value: Self, ast: Option<Self>) -> Self {
        if let Some(mut parent_node) = ast {
            use std::borrow::BorrowMut;
            {
                let mut n = &mut parent_node;
                loop {
                    match n {
                        Self::Conditional(sub) => {
                            n = match sub.kind {
                                ConditionalKind::ModifyingIf | ConditionalKind::ModifyingUnless => {
                                    sub.then.borrow_mut()
                                }
                                _ => sub.cond.borrow_mut(),
                            }
                        }
                        Self::Loop(sub) => {
                            n = match sub.kind {
                                LoopKind::ModifyingWhile | LoopKind::ModifyingUntil => {
                                    sub.body.borrow_mut()
                                }
                                _ => sub.cond.borrow_mut(),
                            }
                        }
                        Self::BinaryOp(sub) => n = sub.lhs.borrow_mut(),
                        Self::LogicalOr(sub) => n = sub.first.borrow_mut(),
                        Self::LogicalAnd(sub) => n = sub.first.borrow_mut(),
                        Self::LogicalNot(sub) => n = sub.expr.borrow_mut(),
                        Self::Rescue(sub) => n = sub.body.borrow_mut(),
                        _ => break,
                    }
                }
                *n = value;
            }
            parent_node
        } else {
            value
        }
    }
}
