use crate::lexer::*;
use crate::parsers::expression::argument::indexing_argument_list;
use crate::parsers::expression::assignment::rhs_expression;
use crate::parsers::expression::primary_expression;
use crate::parsers::expression::variable::variable;
use crate::parsers::token::identifier::constant_identifier;
use crate::parsers::token::identifier::local_variable_identifier;
use crate::parsers::token::operator::assignment_operator;

/// *variable* [ no line terminator here ] *assignment_operator* *rhs_expression*
pub(crate) fn abbreviated_variable_assignment(i: Input) -> NodeResult {
    map(
        tuple((variable, no_lt, assignment_operator, ws, rhs_expression)),
        |_| Node::Placeholder,
    )(i)
}

/// *primary_expression* [ no line terminator here ] [ no whitespace here ] `[` *indexing_argument_list*? `]` [ no line terminator here ] *assignment_operator* *rhs_expression*
pub(crate) fn abbreviated_indexing_assignment(i: Input) -> NodeResult {
    map(
        tuple((
            primary_expression,
            char('['),
            ws,
            indexing_argument_list,
            ws,
            char(']'),
            no_lt,
            assignment_operator,
            ws,
            rhs_expression,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *primary_expression* [ no line terminator here ] ( `.` | `::` ) *local_variable_identifier* [ no line terminator here ] *assignment_operator* *rhs_expression* | *primary_expression* [ no line terminator here ] `.` *constant_identifier* [ no line terimanator here ] *assignment_operator* *rhs_expression*
pub(crate) fn abbreviated_method_assignment(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                primary_expression,
                no_lt,
                alt((tag("."), tag("::"))),
                local_variable_identifier,
                no_lt,
                assignment_operator,
                ws,
                rhs_expression,
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                primary_expression,
                no_lt,
                char('.'),
                constant_identifier,
                no_lt,
                assignment_operator,
                ws,
                rhs_expression,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}
