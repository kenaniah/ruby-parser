use super::*;

#[derive(Debug, PartialEq)]
pub struct Ranged {
    pub from: Box<Node>,
    pub to: Box<Node>,
    pub exclusive: bool,
}
