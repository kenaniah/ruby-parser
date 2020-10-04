use super::*;

#[derive(Debug, PartialEq)]
pub struct Method {
    name: String,
    params: MethodParameters,
    body: Node,
}

#[derive(Debug, PartialEq)]
pub struct MethodParameters {
    pub required: Vec<Parameter>,
    pub optional: Vec<Parameter>,
    pub array: Option<Parameter>,
    pub proc: Option<Parameter>,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<Box<Node>>,
}

#[allow(dead_code)]
impl Parameter {
    pub(crate) fn new_required(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            default_value: None,
        }
    }
    pub(crate) fn new_optional(name: &str, default_value: Node) -> Self {
        Self {
            name: name.to_owned(),
            default_value: Some(Box::new(default_value)),
        }
    }
}
