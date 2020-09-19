use super::*;

#[derive(Debug, PartialEq)]
pub struct Alias {
    pub to: Identifier,
    pub from: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct Undef {
    pub list: Vec<Identifier>,
}

#[derive(Debug, PartialEq)]
pub struct Rescue {
    pub body: Box<Node>,
    pub rescue: Vec<RescueClause>,
    pub otherwise: Box<Node>,
}

#[derive(Debug, PartialEq)]
pub struct RescueClause {
    pub exceptions: Vec<Node>,
    pub assigned_to: Box<Node>,
    pub then: Box<Node>,
}
