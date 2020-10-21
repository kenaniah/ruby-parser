use crate::lexer::*;
use crate::parsers::expression::argument::indexing_argument_list;
use crate::parsers::expression::method::method_invocation_without_parenthesis;
use crate::parsers::expression::operator_expression;
use crate::parsers::expression::primary_expression;
use crate::parsers::expression::variable::variable;
use crate::parsers::token::identifier::constant_identifier;
use crate::parsers::token::identifier::local_variable_identifier;
use crate::parsers::token::operator::assignment_operator;

/// *abbreviated_variable_assignment_expression* | *abbreviated_indexing_assignment_expression* | *abbreviated_method_assignment_expression*
pub(crate) fn abbreviated_assignment_expression(i: Input) -> NodeResult {
    alt((
        abbreviated_variable_assignment_expression,
        abbreviated_indexing_assignment_expression,
        abbreviated_method_assignment_expression,
    ))(i)
}

/// *abbreviated_variable_assignment_statement* | *abbreviated_indexing_assignment_statement* | *abbreviated_method_assignment_statement*
pub(crate) fn abbreviated_assignment_statement(i: Input) -> NodeResult {
    alt((
        abbreviated_variable_assignment_statement,
        abbreviated_indexing_assignment_statement,
        abbreviated_method_assignment_statement,
    ))(i)
}

/// *variable* [ no ⏎ ] *assignment_operator* *operator_expression*
pub(crate) fn abbreviated_variable_assignment_expression(i: Input) -> NodeResult {
    map(
        tuple((
            variable,
            no_lt,
            assignment_operator,
            ws0,
            operator_expression,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *variable* [ no ⏎ ] *assignment_operator* *method_invocation_without_parenthesis*
pub(crate) fn abbreviated_variable_assignment_statement(i: Input) -> NodeResult {
    map(
        tuple((
            variable,
            no_lt,
            assignment_operator,
            ws0,
            method_invocation_without_parenthesis,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *primary_expression* [ no ⏎ ] [ no ⎵ ] `[` *indexing_argument_list*? `]` [ no ⏎ ] *assignment_operator* *operator_expression*
pub(crate) fn abbreviated_indexing_assignment_expression(i: Input) -> NodeResult {
    map(
        tuple((
            primary_expression,
            char('['),
            ws0,
            indexing_argument_list,
            ws0,
            char(']'),
            no_lt,
            assignment_operator,
            ws0,
            operator_expression,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *primary_expression* [ no ⏎ ] [ no ⎵ ] `[` *indexing_argument_list*? `]` [ no ⏎ ] *assignment_operator* *method_invocation_without_parenthesis*
pub(crate) fn abbreviated_indexing_assignment_statement(i: Input) -> NodeResult {
    map(
        tuple((
            primary_expression,
            char('['),
            ws0,
            indexing_argument_list,
            ws0,
            char(']'),
            no_lt,
            assignment_operator,
            ws0,
            method_invocation_without_parenthesis,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *primary_expression* [ no ⏎ ] ( `.` | `::` ) *local_variable_identifier* [ no ⏎ ] *assignment_operator* *operator_expression* | *primary_expression* [ no ⏎ ] `.` *constant_identifier* [ no ⏎ ] *assignment_operator* *operator_expression*
pub(crate) fn abbreviated_method_assignment_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                primary_expression,
                no_lt,
                alt((tag("."), tag("::"))),
                local_variable_identifier,
                no_lt,
                assignment_operator,
                ws0,
                operator_expression,
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
                ws0,
                operator_expression,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}

/// *primary_expression* [ no ⏎ ] ( `.` | `::` ) *local_variable_identifier* [ no ⏎ ] *assignment_operator* *method_invocation_without_parenthesis* | *primary_expression* [ no ⏎ ] `.` *constant_identifier* [ no ⏎ ] *assignment_operator* *method_invocation_without_parenthesis*
pub(crate) fn abbreviated_method_assignment_statement(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                primary_expression,
                no_lt,
                alt((tag("."), tag("::"))),
                local_variable_identifier,
                no_lt,
                assignment_operator,
                ws0,
                method_invocation_without_parenthesis,
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
                ws0,
                method_invocation_without_parenthesis,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}
