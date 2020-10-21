use crate::lexer::*;
use crate::parsers::expression::argument::indexing_argument_list;
use crate::parsers::expression::method::method_invocation_without_parenthesis;
use crate::parsers::expression::operator_expression;
use crate::parsers::expression::primary_expression;
use crate::parsers::expression::variable::variable;
use crate::parsers::token::identifier::constant_identifier;
use crate::parsers::token::identifier::local_variable_identifier;

/// *single_variable_assignment_expression* | *scoped_constant_assignment_expression* | *single_indexing_assignment_expression* | *single_method_assignment_expression*
pub(crate) fn single_assignment_expression(i: Input) -> NodeResult {
    alt((
        single_variable_assignment_expression,
        scoped_constant_assignment_expression,
        single_indexing_assignment_expression,
        single_method_assignment_expression,
    ))(i)
}

/// *single_variable_assignment_statement* | *scoped_constant_assignment_statement* | *single_indexing_assignment_statement* | *single_method_assignment_statement*
pub(crate) fn single_assignment_statement(i: Input) -> NodeResult {
    alt((
        single_variable_assignment_statement,
        scoped_constant_assignment_statement,
        single_indexing_assignment_statement,
        single_method_assignment_statement,
    ))(i)
}

/// *variable* [ no ⏎ ] `=` *operator_expression*
pub(crate) fn single_variable_assignment_expression(i: Input) -> NodeResult {
    map(
        tuple((variable, no_lt, char('='), ws0, operator_expression)),
        |_| Node::Placeholder,
    )(i)
}

/// *variable* [ no ⏎ ] `=` *method_invocation_without_parenthesis*
pub(crate) fn single_variable_assignment_statement(i: Input) -> NodeResult {
    map(
        tuple((
            variable,
            no_lt,
            char('='),
            ws0,
            method_invocation_without_parenthesis,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *primary_expression* [ no ⏎ ] [ no ⎵ ] `[` *indexing_argument_list*? `]` [ no ⏎ ] `=` *operator_expression*
pub(crate) fn single_indexing_assignment_expression(i: Input) -> NodeResult {
    map(
        tuple((
            primary_expression,
            char('['),
            ws0,
            opt(indexing_argument_list),
            ws0,
            char(']'),
            no_lt,
            char('='),
            ws0,
            operator_expression,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *primary_expression* [ no ⏎ ] [ no ⎵ ] `[` *indexing_argument_list*? `]` [ no ⏎ ] `=` *method_invocation_without_parenthesis*
pub(crate) fn single_indexing_assignment_statement(i: Input) -> NodeResult {
    map(
        tuple((
            primary_expression,
            char('['),
            ws0,
            opt(indexing_argument_list),
            ws0,
            char(']'),
            no_lt,
            char('='),
            ws0,
            method_invocation_without_parenthesis,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *primary_expression* [ no ⏎ ] ( `.` | `::` ) *local_variable_identifier* [ no ⏎ ] `=` *operator_expression* | *primary_expression* [ no ⏎ ] `.` *constant_identifier* [ no ⏎ ] `=` *operator_expression*
pub(crate) fn single_method_assignment_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                primary_expression,
                no_lt,
                alt((tag("."), tag("::"))),
                ws0,
                local_variable_identifier,
                no_lt,
                char('='),
                operator_expression,
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                primary_expression,
                no_lt,
                char('.'),
                ws0,
                constant_identifier,
                no_lt,
                char('='),
                operator_expression,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}

/// *primary_expression* [ no ⏎ ] ( `.` | `::` ) *local_variable_identifier* [ no ⏎ ] `=` *method_invocation_without_parenthesis* | *primary_expression* [ no ⏎ ] `.` *constant_identifier* [ no ⏎ ] `=` *method_invocation_without_parenthesis*
pub(crate) fn single_method_assignment_statement(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                primary_expression,
                no_lt,
                alt((tag("."), tag("::"))),
                ws0,
                local_variable_identifier,
                no_lt,
                char('='),
                method_invocation_without_parenthesis,
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                primary_expression,
                no_lt,
                char('.'),
                ws0,
                constant_identifier,
                no_lt,
                char('='),
                method_invocation_without_parenthesis,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}

/// *primary_expression* [ no ⏎ ] [ no ⎵ ] `::` *constant_identifier* [ no ⏎ ] `=` *operator_expression* | `::` *constant_identifier* [ no ⏎ ] `=` *operator_expression*
pub(crate) fn scoped_constant_assignment_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                tag("::"),
                constant_identifier,
                no_lt,
                char('='),
                ws0,
                operator_expression,
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                primary_expression,
                tag("::"),
                constant_identifier,
                no_lt,
                char('='),
                ws0,
                operator_expression,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}

/// *primary_expression* [ no ⏎ ] [ no ⎵ ] `::` *constant_identifier* [ no ⏎ ] `=` *method_invocation_without_parenthesis* | `::` *constant_identifier* [ no ⏎ ] `=` *method_invocation_without_parenthesis*
pub(crate) fn scoped_constant_assignment_statement(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                tag("::"),
                constant_identifier,
                no_lt,
                char('='),
                ws0,
                method_invocation_without_parenthesis,
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                primary_expression,
                tag("::"),
                constant_identifier,
                no_lt,
                char('='),
                ws0,
                method_invocation_without_parenthesis,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}
