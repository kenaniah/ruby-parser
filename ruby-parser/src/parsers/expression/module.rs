use crate::lexer::*;

/// `module` *module_path* *module_body* `end`
pub(crate) fn module_definition(i: Input) -> NodeResult {
    stub(i)
}

/// *top_module_path* | *module_name* | *nested_module_path*
pub(crate) fn module_path(i: Input) -> NodeResult {
    stub(i)
}

/// *constant_identifier*
pub(crate) fn module_name(i: Input) -> NodeResult {
    stub(i)
}

/// `::` *module_name*
pub(crate) fn top_module_path(i: Input) -> NodeResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] `::` *module_name*
pub(crate) fn nested_module_path(i: Input) -> NodeResult {
    stub(i)
}

/// *body_statement*
pub(crate) fn module_body(i: Input) -> NodeResult {
    stub(i)
}
