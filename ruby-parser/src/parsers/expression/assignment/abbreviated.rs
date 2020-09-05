use crate::lexer::*;

/// *variable* [ no line terminator here ] *assignment_operator* *rhs_expression*
pub(crate) fn abbreviated_variable_assignment(i: Input) -> NodeResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] [ no whitespace here ] `[` *indexing_argument_list*? `]` [ no line terminator here ] *assignment_operator* *rhs_expression*
pub(crate) fn abbreviated_indexing_assignment(i: Input) -> NodeResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] ( `.` | `::` ) *local_variable_identifier* [ no line terminator here ] *assignment_operator* *rhs_expression* | *primary_expression* [ no line terminator here ] `.` *constant_identifier* [ no line terimanator here ] *assignment_operator* *rhs_expression*
pub(crate) fn abbreviated_method_assignment(i: Input) -> NodeResult {
    stub(i)
}
