use super::*;

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<Box<Node>>,
}
