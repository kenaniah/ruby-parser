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
