use super::*;

#[derive(Debug, PartialEq)]
pub struct Alias {
    pub to: Box<Node>,
    pub from: Box<Node>,
}
