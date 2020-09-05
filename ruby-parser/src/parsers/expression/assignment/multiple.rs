use crate::lexer::*;

/// *many_to_one_assignment_statement* | *one_to_packing_assignment_statement* | *many_to_many_assignment_statement*
pub(crate) fn multiple_assignment_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *left_hand_side* [ no line terminator here ] `=` *multiple_right_hand_side*
pub(crate) fn many_to_one_assignment_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *packing_left_hand_side* [ no line terminator here ] `=` *rhs_expression*
pub(crate) fn one_to_packing_assignment_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *multiple_left_hand_side* [ no line terminator here ] `=` *multiple_right_hand_side* | ( *multiple_left_hand_side* **but not** *packing_left_hand_side* ) [ no line terminator here ] `=` *rhs_expression*
pub(crate) fn many_to_many_assignment_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *variable* | *primary_expression* [ no line terminator here ] [ no whitespace here ] `[` *indexing_argument_list*? `]` | *primary_expression* [ no line terminator here ] ( `.` | `::` ) ( *local_variable_identifier* | *constant_identifier* ) | `::` *constant_identifier*
pub(crate) fn left_hand_side(i: Input) -> NodeResult {
    stub(i)
}

/// ( *multiple_left_hand_side_item* [ no line terminator here ] `,` )+ *multiple_left_hand_side_item*? | ( *multiple_left_hand_side_item* [ no line terminator here ] `,` )+ *packing_left_hand_side*? | *packing_left_hand_side* | *grouped_left_hand_side*
pub(crate) fn multiple_left_hand_side(i: Input) -> NodeResult {
    stub(i)
}

/// `*` *left_hand_side*?
pub(crate) fn packing_left_hand_side(i: Input) -> NodeResult {
    stub(i)
}

/// `(` *multiple_left_hand_side* `)`
pub(crate) fn grouped_left_hand_side(i: Input) -> NodeResult {
    stub(i)
}

/// *left_hand_side* | *grouped_left_hand_side*
pub(crate) fn multiple_left_hand_side_item(i: Input) -> NodeResult {
    stub(i)
}

/// *operator_expression_list* ( [ no line terminator here ] `,` *splatting_right_hand_side* )? | *splatting_right_hand_side*
pub(crate) fn multiple_right_hand_side(i: Input) -> NodeResult {
    stub(i)
}

/// *splatting_argument*
pub(crate) fn splatting_right_hand_side(i: Input) -> NodeResult {
    stub(i)
}
