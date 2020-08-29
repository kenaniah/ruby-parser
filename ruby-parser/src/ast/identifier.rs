#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub kind: IdentifierKind,
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
