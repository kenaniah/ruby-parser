use crate::lexer::*;
use crate::parsers::expression::argument::operator_expression_list;
use crate::parsers::expression::argument::splatting_argument;

/// *many_to_one_assignment_statement* | *one_to_packing_assignment_statement* | *many_to_many_assignment_statement*
pub(crate) fn multiple_assignment_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *left_hand_side* [ no ⏎ ] `=` *multiple_right_hand_side*
pub(crate) fn many_to_one_assignment_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *packing_left_hand_side* [ no ⏎ ] `=` *rhs_expression*
pub(crate) fn one_to_packing_assignment_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *multiple_left_hand_side* [ no ⏎ ] `=` *multiple_right_hand_side* | ( *multiple_left_hand_side* **but not** *packing_left_hand_side* ) [ no ⏎ ] `=` *rhs_expression*
pub(crate) fn many_to_many_assignment_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *variable* | *primary_expression* [ no ⏎ ] [ no ⎵ ] `[` *indexing_argument_list*? `]` | *primary_expression* [ no ⏎ ] ( `.` | `::` ) ( *local_variable_identifier* | *constant_identifier* ) | `::` *constant_identifier*
pub(crate) fn left_hand_side(i: Input) -> NodeResult {
    stub(i)
}

/// ( *multiple_left_hand_side_item* [ no ⏎ ] `,` )+ *multiple_left_hand_side_item*? | ( *multiple_left_hand_side_item* [ no ⏎ ] `,` )+ *packing_left_hand_side*? | *packing_left_hand_side* | *grouped_left_hand_side*
pub(crate) fn multiple_left_hand_side(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                many1(tuple((multiple_left_hand_side_item, no_lt, char(',')))),
                opt(alt((multiple_left_hand_side_item, packing_left_hand_side))),
            )),
            |_| Node::Placeholder,
        ),
        packing_left_hand_side,
        grouped_left_hand_side,
    ))(i)
}

/// `*` *left_hand_side*?
pub(crate) fn packing_left_hand_side(i: Input) -> NodeResult {
    map(tuple((char('*'), ws, opt(left_hand_side))), |_| {
        Node::Placeholder
    })(i)
}

/// `(` *multiple_left_hand_side* `)`
pub(crate) fn grouped_left_hand_side(i: Input) -> NodeResult {
    map(
        tuple((char('('), ws, multiple_left_hand_side, ws, char(')'))),
        |_| Node::Placeholder,
    )(i)
}

/// *left_hand_side* | *grouped_left_hand_side*
pub(crate) fn multiple_left_hand_side_item(i: Input) -> NodeResult {
    alt((left_hand_side, grouped_left_hand_side))(i)
}

/// *operator_expression_list* ( [ no ⏎ ] `,` *splatting_right_hand_side* )? | *splatting_right_hand_side*
pub(crate) fn multiple_right_hand_side(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                operator_expression_list,
                opt(tuple((no_lt, char(','), ws, splatting_right_hand_side))),
            )),
            |_| Node::Placeholder,
        ),
        splatting_right_hand_side,
    ))(i)
}

/// *splatting_argument*
pub(crate) fn splatting_right_hand_side(i: Input) -> NodeResult {
    splatting_argument(i)
}
