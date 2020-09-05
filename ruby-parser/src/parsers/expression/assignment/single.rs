use crate::lexer::*;

/// *single_variable_assignment* | *scoped_constant_assignment* | *single_indexing_assignment* | *single_method_assignment*
pub(crate) fn single_assignment(i: Input) -> NodeResult {
    stub(i)
}

/// *variable* [ no line terminator here ] `=` *rhs_expression*
pub(crate) fn single_variable_assignment(i: Input) -> NodeResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] [ no whitespace here ] `[` *indexing_argument_list*? `]` [ no line terminator here ] `=` *rhs_expression*
pub(crate) fn single_indexing_assignment(i: Input) -> NodeResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] ( `.` | `::` ) *local_variable_identifier* [ no line terminator here ] `=` *rhs_expression* | *primary_expression* [ no line terminator here ] `.` *constant_identifier* [ no line terimanator here ] `=` *rhs_expression*
pub(crate) fn single_method_assignment(i: Input) -> NodeResult {
    stub(i)
}
