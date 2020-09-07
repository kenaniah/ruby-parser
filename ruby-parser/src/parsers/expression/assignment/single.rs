use crate::lexer::*;

/// *single_variable_assignment* | *scoped_constant_assignment* | *single_indexing_assignment* | *single_method_assignment*
pub(crate) fn single_assignment(i: Input) -> NodeResult {
    stub(i)
}

/// *variable* [ no ⏎ ] `=` *rhs_expression*
pub(crate) fn single_variable_assignment(i: Input) -> NodeResult {
    stub(i)
}

/// *primary_expression* [ no ⏎ ] [ no ⎵ ] `[` *indexing_argument_list*? `]` [ no ⏎ ] `=` *rhs_expression*
pub(crate) fn single_indexing_assignment(i: Input) -> NodeResult {
    stub(i)
}

/// *primary_expression* [ no ⏎ ] ( `.` | `::` ) *local_variable_identifier* [ no ⏎ ] `=` *rhs_expression* | *primary_expression* [ no ⏎ ] `.` *constant_identifier* [ no line terimanator here ] `=` *rhs_expression*
pub(crate) fn single_method_assignment(i: Input) -> NodeResult {
    stub(i)
}
