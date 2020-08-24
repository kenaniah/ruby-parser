use crate::lexer::*;

/// *super_with_optional_argument* | *indexing_method_invocation* | *method_only_identifier* | *method_identifier* *block* | *method_identifier* [ no line terminator here ] [ no whitespace here ] *argument_with_parentheses* *block*? | *primary_expression* [ no line terminator here ] `.` *method_name* ( [ no line terminator here ] [ no whitespace here ] *argument_with_parentheses* )? *block*? | *primary_expression* [ no line terminator here ] `::` *method_name* [ no line terminator here ] [ no whitespace here ] *argument_with_parentheses* *block*? | *primary_expression* [ no line terminator here ] `::` *method_name_except_constant* *block*?
pub(crate) fn primary_method_invocation(i: Input) -> TokenResult {
    stub(i)
}

/// *local_variable_identifier* | *constant_identifier* | *method_only_identifier*
pub(crate) fn method_identifier(i: Input) -> TokenResult {
    stub(i)
}

/// *method_identifier* | *operator_method_name* | *keyword*
pub(crate) fn method_name(i: Input) -> TokenResult {
    stub(i)
}

/// *primary_expression* [ no line terminator here ] [ no whitespace here ] `[` *indexing_argument_list*? `]`
pub(crate) fn indexing_method_invocation(i: Input) -> TokenResult {
    stub(i)
}

/// *method_name* **but not** *constant_identifier*
pub(crate) fn method_name_except_constant(i: Input) -> TokenResult {
    stub(i)
}

/// *command* | *chained_command_with_do_block* | *chained_command_with_do_block* ( `.` | `::` ) *method_name* *argument_without_parenthesis* | *return_with_argument* | *break_with_argument* | *next_with_argument*
pub(crate) fn method_invocation_without_parenthesis(i: Input) -> TokenResult {
    stub(i)
}

/// *super_with_argument* | *yield_with_argument* | *method_identifier* *argument_without_parenthesis* | *primary_expression* [ no line terminator here ] ( `.` | `::` ) *method_name* *argument_without_parenthesis*
pub(crate) fn command(i: Input) -> TokenResult {
    stub(i)
}

/// *command_with_do_block* *chained_method_invocation**
pub(crate) fn chained_command_with_do_block(i: Input) -> TokenResult {
    stub(i)
}

/// ( `.` | `::` ) *method_name* | ( `.` | `::` ) *method_name* [ no line terminator here ] [ no whitespace here ] *argument_with_parentheses*
pub(crate) fn chained_method_invocation(i: Input) -> TokenResult {
    stub(i)
}

/// *super_with_argument_and_do_block* | *method_identifier* *argument_without_parenthesis* *do_block* | *primary_expression* [ no line terminator here ] ( `.` | `::` ) *method_name* *argument_without_parenthesis* *do_block*
pub(crate) fn command_with_do_block(i: Input) -> TokenResult {
    stub(i)
}

fn stub(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
