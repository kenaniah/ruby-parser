use super::*;

#[derive(Debug, PartialEq)]
pub enum Interpolated {
    Command(Vec<Node>),
    String(Vec<Node>),
    Symbol(Vec<Node>),
}
