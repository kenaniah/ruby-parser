use super::*;

#[derive(Debug, PartialEq)]
pub struct Loop {
    pub kind: LoopKind,
    pub cond: Box<Node>,
    pub body: Box<Node>,
    pub bindings: Option<Vec<Node>>,
}

#[derive(Debug, PartialEq)]
pub enum LoopKind {
    For,
    While,
    Until,
    ModifyingWhile,
    ModifyingUntil,
}
