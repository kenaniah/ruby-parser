use crate::lexer::*;
use crate::parsers::expression::begin::body_statement;

/// `class` `<<` *expression* *separator* *singleton_class_body* `end`
pub(crate) fn singleton_class_definition(i: Input) -> NodeResult {
    stub(i)
}

/// *body_statement*
pub(crate) fn singleton_class_body(i: Input) -> NodeResult {
    body_statement(i)
}

/// `def` *singleton* ( `.` | `::` ) *defined_method_name* [ no ⏎ ] *method_parameter_part* *method_body* `end`
pub(crate) fn singleton_method_definition(i: Input) -> NodeResult {
    stub(i)
}

/// *variable_reference* | `(` *expression* `)`
pub(crate) fn singleton(i: Input) -> NodeResult {
    stub(i)
}
