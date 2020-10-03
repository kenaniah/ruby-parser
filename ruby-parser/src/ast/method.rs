use super::*;

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub default_value: Option<Box<Node>>,
}

#[allow(dead_code)]
impl Parameter {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            default_value: None,
        }
    }
    pub(crate) fn new_with_default(name: &str, default_value: Node) -> Self {
        Self {
            name: name.to_owned(),
            default_value: Some(Box::new(default_value)),
        }
    }
}
