use crate::lexer::TokenResult;
use crate::Input;

/// *single_assignment* | *abbreviated_assignment_expression* | *assignment_with_rescue_modifier*
pub(crate) fn assignment_expression(i: Input) -> TokenResult {
    stub(i)
}

/// *single_assignment* | *abbreviated_assignment_statement* | *multiple_assignment_statement*
pub(crate) fn assignment_statement(i: Input) -> TokenResult {
    stub(i)
}

/// *single_variable_assignment* | *scoped_constant_assignment* | *single_indexing_assignment* | *single_method_assignment*
pub(crate) fn single_assignment(i: Input) -> TokenResult {
    stub(i)
}

/// *variable* [ no line terminator here ] `=` *rhs_expression*
pub(crate) fn single_variable_assignment(i: Input) -> TokenResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] [ no whitespace here ] `::` *constant_identifier* [ no line terminator here ] `=` *rhs_expression* | `::` *constant_identifier* [ no line terminator here ] `=` *rhs_expression*
pub(crate) fn scoped_constant_assignment(i: Input) -> TokenResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] [ no whitespace here ] `[` *indexing_argument_list*? `]` [ no line terminator here ] `=` *rhs_expression*
pub(crate) fn single_indexing_assignment(i: Input) -> TokenResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] ( `.` | `::` ) *local_variable_identifier* [ no line terminator here ] `=` *rhs_expression* | *primary_expression* [ no line terminator here ] `.` *constant_identifier* [ no line terimanator here ] `=` *rhs_expression*
pub(crate) fn single_method_assignment(i: Input) -> TokenResult {
    stub(i)
}

/// *variable* [ no line terminator here ] *assignment_operator* *rhs_expression*
pub(crate) fn abbreviated_variable_assignment(i: Input) -> TokenResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] [ no whitespace here ] `[` *indexing_argument_list*? `]` [ no line terminator here ] *assignment_operator* *rhs_expression*
pub(crate) fn abbreviated_indexing_assignment(i: Input) -> TokenResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] ( `.` | `::` ) *local_variable_identifier* [ no line terminator here ] *assignment_operator* *rhs_expression* | *primary_expression* [ no line terminator here ] `.` *constant_identifier* [ no line terimanator here ] *assignment_operator* *rhs_expression*
pub(crate) fn abbreviated_method_assignment(i: Input) -> TokenResult {
    stub(i)
}

/// *many_to_one_assignment_statement* | *one_to_packing_assignment_statement* | *many_to_many_assignment_statement*
pub(crate) fn multiple_assignment_statement(i: Input) -> TokenResult {
    stub(i)
}

// TODO: fill out 11.4.2.4

/// *left_hand_side* [ no line terminator here ] `=` *operator_expression* [ no line terminator here ] `rescue` *operator_expression*
pub(crate) fn assignment_with_rescue_modifier(i: Input) -> TokenResult {
    stub(i)
}

/// ( *operator_expression* | *method_invocation_without_parenthesis* )
pub(crate) fn rhs_expression(i: Input) -> TokenResult {
    stub(i)
}

fn stub(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
