use super::*;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub kind: IdentifierKind,
}

impl Identifier {
    pub fn new(name: String, kind: IdentifierKind) -> Self {
        Self { name, kind }
    }
}

impl From<Identifier> for String {
    fn from(v: Identifier) -> Self {
        v.name
    }
}

impl From<Identifier> for Node {
    fn from(v: Identifier) -> Self {
        Self::Identifier(v)
    }
}

#[derive(Debug, PartialEq)]
pub enum IdentifierKind {
    LocalVariable,
    GlobalVariable,
    ClassVariable,
    InstanceVariable,
    Constant,
    Method,
    AssignmentMethod,
}
