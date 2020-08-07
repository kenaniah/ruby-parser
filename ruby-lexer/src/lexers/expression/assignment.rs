use crate::*;

/// *single_assignment_expression* | *abbreviated_assignment_expression* | *assignment_with_rescue_modifier*
pub(crate) fn assignment_expression(i: Input) -> TokenResult {
    stub(i)
}

/// *single_assignment_statement* | *abbreviated_assignment_statement* | *multiple_assignment_statement*
pub(crate) fn assignment_statement(i: Input) -> TokenResult {
    stub(i)
}

/// *single_variable_assignment_expression* | *scoped_constant_assignment_expression* | *single_indexing_assignment_expression* | *single_method_assignment_expression*
pub(crate) fn single_assignment_expression(i: Input) -> TokenResult {
    stub(i)
}

/// *single_variable_assignment_statement* | *scoped_constant_assignment_statement* | *single_indexing_assignment_statement* | *single_method_assignment_statement*
pub(crate) fn single_assignment_statement(i: Input) -> TokenResult {
    stub(i)
}

/// *variable* [ no line terminator here ] `=` *operator_expression*
pub(crate) fn single_variable_assignment_expression(i: Input) -> TokenResult {
    stub(i)
}

/// *variable* [ no line terminator here ] `=` *method_invocation_without_parenthesis*
pub(crate) fn single_variable_assignment_statement(i: Input) -> TokenResult {
    stub(i)
}

// scoped-constant-assignment-expression is next

fn stub(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
