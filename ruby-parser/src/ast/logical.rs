use super::*;

pub struct LogicalAnd {
    pub first: Box<Node>,
    pub second: Box<Node>,
}

pub struct LogicalOr {
    pub first: Box<Node>,
    pub second: Box<Node>,
}

pub struct LogicalNot {
    pub expr: Box<Node>
}
