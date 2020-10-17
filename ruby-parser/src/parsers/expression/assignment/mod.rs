use crate::lexer::*;
use crate::parsers::expression::assignment::multiple::left_hand_side;
use crate::parsers::expression::assignment::multiple::multiple_assignment_statement;
use crate::parsers::expression::operator_expression;

pub(crate) mod abbreviated;
pub(crate) mod multiple;
pub(crate) mod single;

// TODO: figure out if assignment expressions and statements can be gramatically combined

/// *single_assignment_expression* | *abbreviated_assignment_expression* | *assignment_with_rescue_modifier*
pub(crate) fn assignment_expression(i: Input) -> NodeResult {
    alt((
        single::single_assignment_expression,
        abbreviated::abbreviated_assignment_expression,
        assignment_with_rescue_modifier,
    ))(i)
}

/// *single_assignment_statement* | *abbreviated_assignment_statement* | *multiple_assignment_statement*
pub(crate) fn assignment_statement(i: Input) -> NodeResult {
    alt((
        single::single_assignment_statement,
        abbreviated::abbreviated_assignment_statement,
        multiple_assignment_statement,
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
