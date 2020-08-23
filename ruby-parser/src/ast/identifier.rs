pub struct Identifier {
    pub name: String,
    pub ty: IdentifierType,
}

pub enum IdentifierType {
    LocalVariable,
    GlobalVariable,
    ClassVariable,
    InstanceVariable,
    Constant,
    Method,
    AssignmentMethod,
}
