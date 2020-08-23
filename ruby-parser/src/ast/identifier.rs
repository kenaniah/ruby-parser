#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub ty: IdentifierType,
}

#[derive(Debug, PartialEq)]
pub enum IdentifierType {
    LocalVariable,
    GlobalVariable,
    ClassVariable,
    InstanceVariable,
    Constant,
    Method,
    AssignmentMethod,
}
