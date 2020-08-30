use super::*;

#[derive(Debug, PartialEq)]
pub struct Conditional {
    pub cond: Box<Node>,
    pub then: Option<Box<Node>>,
    pub otherwise: Option<Box<Node>>,
}

#[derive(Debug, PartialEq)]
pub struct LogicalAnd {
    pub first: Box<Node>,
    pub second: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct LogicalOr {
    pub first: Box<Node>,
    pub second: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct LogicalNot {
    pub expr: Box<Node>,
}
