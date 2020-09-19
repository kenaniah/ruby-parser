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
