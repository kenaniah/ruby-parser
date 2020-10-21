use crate::lexer::*;
use crate::parsers::expression::argument::indexing_argument_list;
use crate::parsers::expression::argument::operator_expression_list;
use crate::parsers::expression::argument::splatting_argument;
use crate::parsers::expression::method::method_invocation_without_parenthesis;
use crate::parsers::expression::operator_expression;
use crate::parsers::expression::primary_expression;
use crate::parsers::expression::variable::variable;
use crate::parsers::token::identifier::constant_identifier;
use crate::parsers::token::identifier::local_variable_identifier;

/// *many_to_one_assignment_statement* | *one_to_packing_assignment_statement* | *many_to_many_assignment_statement*
pub(crate) fn multiple_assignment_statement(i: Input) -> NodeResult {
    alt((
        many_to_one_assignment_statement,
        one_to_packing_assignment_statement,
        many_to_many_assignment_statement,
    ))(i)
}

/// *left_hand_side* [ no ⏎ ] `=` *multiple_right_hand_side*
pub(crate) fn many_to_one_assignment_statement(i: Input) -> NodeResult {
    map(
        tuple((
            left_hand_side,
            no_lt,
            char('='),
            ws0,
            multiple_right_hand_side,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *packing_left_hand_side* [ no ⏎ ] `=` *rhs_expression*
pub(crate) fn one_to_packing_assignment_statement(i: Input) -> NodeResult {
    map(
        tuple((
            packing_left_hand_side,
            no_lt,
            char('='),
            ws0,
            rhs_expression,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *multiple_left_hand_side* [ no ⏎ ] `=` *multiple_right_hand_side* | ( *multiple_left_hand_side* **but not** *packing_left_hand_side* ) [ no ⏎ ] `=` *rhs_expression*
pub(crate) fn many_to_many_assignment_statement(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                multiple_left_hand_side,
                no_lt,
                char('='),
                ws0,
                multiple_right_hand_side,
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                peek(not(packing_left_hand_side)),
                multiple_left_hand_side,
                no_lt,
                char('='),
                ws0,
                rhs_expression,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}

/// *method_invocation_without_parenthesis* | *operator_expression*
pub(crate) fn rhs_expression(i: Input) -> NodeResult {
    alt((method_invocation_without_parenthesis, operator_expression))(i)
}

/// *variable* | *primary_expression* [ no ⏎ ] [ no ⎵ ] `[` *indexing_argument_list*? `]` | *primary_expression* [ no ⏎ ] ( `.` | `::` ) ( *local_variable_identifier* | *constant_identifier* ) | `::` *constant_identifier*
pub(crate) fn left_hand_side(i: Input) -> NodeResult {
    alt((
        map(variable, |_| Node::Placeholder),
        map(
            tuple((
                primary_expression,
                char('['),
                ws0,
                opt(indexing_argument_list),
                ws0,
                char(']'),
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                primary_expression,
                no_lt,
                alt((tag("."), tag("::"))),
                ws0,
                alt((local_variable_identifier, constant_identifier)),
            )),
            |_| Node::Placeholder,
        ),
        map(tuple((tag("::"), ws0, constant_identifier)), |_| {
            Node::Placeholder
        }),
    ))(i)
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
    map(tuple((char('*'), ws0, opt(left_hand_side))), |_| {
        Node::Placeholder
    })(i)
}

/// `(` *multiple_left_hand_side* `)`
pub(crate) fn grouped_left_hand_side(i: Input) -> NodeResult {
    map(
        tuple((char('('), ws0, multiple_left_hand_side, ws0, char(')'))),
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
                opt(tuple((no_lt, char(','), ws0, splatting_right_hand_side))),
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
