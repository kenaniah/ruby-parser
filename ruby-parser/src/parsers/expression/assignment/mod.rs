use crate::lexer::*;
use crate::parsers::expression::assignment::abbreviated::abbreviated_variable_assignment;
use crate::parsers::expression::assignment::multiple::left_hand_side;
use crate::parsers::expression::assignment::multiple::multiple_assignment_statement;
use crate::parsers::expression::assignment::single::single_assignment;
use crate::parsers::expression::method::method_invocation_without_parenthesis;
use crate::parsers::expression::operator_expression;
use crate::parsers::expression::primary_expression;
use crate::parsers::token::identifier::constant_identifier;

pub(crate) mod abbreviated;
pub(crate) mod multiple;
pub(crate) mod single;

// TODO: figure out if assignment expressions and statements can be gramatically combined

/// *single_assignment* | *abbreviated_assignment_expression* | *assignment_with_rescue_modifier*
pub(crate) fn assignment_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *single_assignment* | *abbreviated_assignment_statement* | *multiple_assignment_statement*
pub(crate) fn assignment_statement(i: Input) -> NodeResult {
    alt((
        single_assignment,
        abbreviated_variable_assignment,
        multiple_assignment_statement,
    ))(i)
}

/// *primary_expression* [ no ⏎ ] [ no ⎵ ] `::` *constant_identifier* [ no ⏎ ] `=` *rhs_expression* | `::` *constant_identifier* [ no ⏎ ] `=` *rhs_expression*
pub(crate) fn scoped_constant_assignment(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                tag("::"),
                constant_identifier,
                no_lt,
                char('='),
                ws0,
                rhs_expression,
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
                rhs_expression,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}

/// *left_hand_side* [ no ⏎ ] `=` *operator_expression* [ no ⏎ ] `rescue` *operator_expression*
pub(crate) fn assignment_with_rescue_modifier(i: Input) -> NodeResult {
    map(
        tuple((
            left_hand_side,
            no_lt,
            char('='),
            ws0,
            operator_expression,
            no_lt,
            tag("rescue"),
            ws0,
            operator_expression,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *operator_expression* | *method_invocation_without_parenthesis*
pub(crate) fn rhs_expression(i: Input) -> NodeResult {
    alt((operator_expression, method_invocation_without_parenthesis))(i)
}
