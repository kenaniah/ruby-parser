use super::*;

#[derive(Debug, PartialEq)]
pub struct Conditional {
    pub kind: ConditionalKind,
    pub cond: Box<Node>,
    pub then: Box<Node>,
    pub otherwise: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub enum ConditionalKind {
    If,
    Unless,
    Ternary,
    Elsif,
    ModifyingIf,
    ModifyingUnless,
}
