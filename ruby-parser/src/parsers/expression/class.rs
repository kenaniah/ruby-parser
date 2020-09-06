use crate::lexer::*;

/// `class` *module_path* [ no line terminator here ] ( `<` *superclass* )? *module_body* `end`
pub(crate) fn class_definition(i: Input) -> NodeResult {
    stub(i)
}

/// *expression*
pub(crate) fn superclass(i: Input) -> NodeResult {
    stub(i)
}
