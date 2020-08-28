use super::*;

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
    pub expr: Box<Node>
}
