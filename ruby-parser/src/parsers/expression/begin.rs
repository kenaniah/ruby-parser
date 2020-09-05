use crate::lexer::*;

/// `begin` *body_statement* `end`
pub(crate) fn begin_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *compound_statement* *rescue_clause** *else_clause*? *ensure_clause*?
pub(crate) fn body_statement(i: Input) -> NodeResult {
    stub(i)
}

/// `rescue` [ no line terminator here ] *exception_class_list*? *exception_variable_assignment*? *then_clause*
pub(crate) fn rescue_clause(i: Input) -> NodeResult {
    stub(i)
}

/// *operator_expression* | *multiple_right_hand_side*
pub(crate) fn exception_class_list(i: Input) -> NodeResult {
    stub(i)
}

/// `=>` *left_hand_side*
pub(crate) fn exception_variable_assignment(i: Input) -> NodeResult {
    stub(i)
}

/// `ensure` *compound_statement*
pub(crate) fn ensure_clause(i: Input) -> NodeResult {
    stub(i)
}
