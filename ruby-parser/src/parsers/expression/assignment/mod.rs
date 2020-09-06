use crate::lexer::*;

pub(crate) mod abbreviated;
pub(crate) mod multiple;
pub(crate) mod single;

/// *single_assignment* | *abbreviated_assignment_expression* | *assignment_with_rescue_modifier*
pub(crate) fn assignment_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *single_assignment* | *abbreviated_assignment_statement* | *multiple_assignment_statement*
pub(crate) fn assignment_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] [ no whitespace here ] `::` *constant_identifier* [ no line terminator here ] `=` *rhs_expression* | `::` *constant_identifier* [ no line terminator here ] `=` *rhs_expression*
pub(crate) fn scoped_constant_assignment(i: Input) -> NodeResult {
    stub(i)
}

/// *left_hand_side* [ no line terminator here ] `=` *operator_expression* [ no line terminator here ] `rescue` *operator_expression*
pub(crate) fn assignment_with_rescue_modifier(i: Input) -> NodeResult {
    stub(i)
}

/// ( *operator_expression* | *method_invocation_without_parenthesis* )
pub(crate) fn rhs_expression(i: Input) -> NodeResult {
    stub(i)
}
