use super::*;

#[derive(Debug, PartialEq)]
pub struct Alias {
    pub to: String,
    pub from: String,
}

#[derive(Debug, PartialEq)]
pub struct Undef {
    pub list: Vec<String>,
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
