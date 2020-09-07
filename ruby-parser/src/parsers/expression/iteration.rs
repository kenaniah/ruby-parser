use crate::lexer::*;
use crate::parsers::expression::assignment::multiple::left_hand_side;
use crate::parsers::expression::assignment::multiple::multiple_left_hand_side;
use crate::parsers::expression::expression;
use crate::parsers::program::compound_statement;
use crate::parsers::program::separator;

/// `while` *expression* *do_clause* `end`
pub(crate) fn while_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("while"), expression, do_clause, tag("end"))),
        |_| Node::Placeholder,
    )(i)
}

/// *separator* *compound_statement* | [ no ⏎ ] `do` *compound_statement*
pub(crate) fn do_clause(i: Input) -> NodeResult {
    alt((
        map(tuple((separator, compound_statement)), |t| {
            Node::Placeholder
        }),
        map(tuple((no_lt, tag("do"), compound_statement)), |t| {
            Node::Placeholder
        }),
    ))(i)
}

/// `until` *expression* *do_clause* `end`
pub(crate) fn until_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("until"), expression, do_clause, tag("end"))),
        |_| Node::Placeholder,
    )(i)
}

/// `for` *for_variable* [ no ⏎ ] `in` *expression* *do_clause* `end`
pub(crate) fn for_expression(i: Input) -> NodeResult {
    map(
        tuple((
            tag("for"),
            for_variable,
            no_lt,
            tag("in"),
            expression,
            do_clause,
            tag("end"),
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *left_hand_side* | *multiple_left_hand_side*
pub(crate) fn for_variable(i: Input) -> NodeResult {
    alt((left_hand_side, multiple_left_hand_side))(i)
}
