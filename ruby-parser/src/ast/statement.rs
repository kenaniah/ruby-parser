use super::*;

#[derive(Debug, PartialEq)]
pub struct Alias {
    pub to: Identifier,
    pub from: Identifier,
}
