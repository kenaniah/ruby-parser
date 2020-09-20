use crate::lexer::*;
use crate::parsers::expression::assignment::multiple::left_hand_side;
use crate::parsers::expression::assignment::multiple::multiple_right_hand_side;
use crate::parsers::expression::conditional::{else_clause, then_clause};
use crate::parsers::expression::operator_expression;
use crate::parsers::program::compound_statement;

/// `begin` *body_statement* `end`
pub(crate) fn begin_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("begin"), body_statement, tag("end"))),
        |_| Node::Placeholder,
    )(i)
}

/// *compound_statement* *rescue_clause** *else_clause*? *ensure_clause*?
pub(crate) fn body_statement(i: Input) -> NodeResult {
    map(
        tuple((
            compound_statement,
            many0(rescue_clause),
            opt(else_clause),
            opt(ensure_clause),
        )),
        |_| Node::Placeholder,
    )(i)
}

/// `rescue` [ no ⏎ ] *exception_class_list*? *exception_variable_assignment*? *then_clause*
pub(crate) fn rescue_clause(i: Input) -> NodeResult {
    map(
        tuple((
            tag("rescue"),
            no_lt,
            opt(exception_class_list),
            opt(exception_variable_assignment),
            then_clause,
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *operator_expression* | *multiple_right_hand_side*
pub(crate) fn exception_class_list(i: Input) -> NodeResult {
    alt((multiple_right_hand_side, operator_expression))(i)
}

/// `=>` *left_hand_side*
pub(crate) fn exception_variable_assignment(i: Input) -> NodeResult {
    map(tuple((tag("=>"), ws0, left_hand_side)), |t| t.2)(i)
}

/// `ensure` *compound_statement*
pub(crate) fn ensure_clause(i: Input) -> NodeResult {
    map(tuple((tag("ensure"), compound_statement)), |t| t.1)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_clause() {
        use_parser!(ensure_clause);
        assert_ok!("ensure ", Node::empty());
        assert_ok!("ensure 2; 5", Node::Block(vec![Node::int(2), Node::int(5)]));
    }
}
