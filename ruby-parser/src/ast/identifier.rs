use crate::lexer::Node;
use core::convert::TryFrom;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub kind: IdentifierKind,
}

impl TryFrom<Node> for Identifier {
    type Error = &'static str;
    fn try_from(value: Node) -> Result<Self, Self::Error> {
        if let Node::Identifier(identifier) = value {
            Ok(identifier)
        } else {
            Err("Could not convert node into an identifier.")
        }
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
