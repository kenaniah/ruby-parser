use crate::ast::{Loop, LoopKind};
use crate::lexer::*;
use crate::parsers::expression::assignment::multiple::left_hand_side;
use crate::parsers::expression::assignment::multiple::multiple_left_hand_side;
use crate::parsers::expression::expression;
use crate::parsers::program::compound_statement;
use crate::parsers::program::separator;

/// `while` *expression* *do_clause* `end`
pub(crate) fn while_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("while"), ws0, expression, do_clause, tag("end"))),
        |t| {
            Node::Loop(Loop {
                kind: LoopKind::While,
                cond: Box::new(t.2),
                body: Box::new(t.3),
                bindings: None,
            })
        },
    )(i)
}

/// *separator* *compound_statement* | [ no ⏎ ] `do` *compound_statement*
pub(crate) fn do_clause(i: Input) -> NodeResult {
    alt((
        map(tuple((separator, compound_statement)), |t| t.1),
        map(tuple((no_lt, tag("do"), compound_statement)), |t| t.2),
    ))(i)
}

/// `until` *expression* *do_clause* `end`
pub(crate) fn until_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("until"), ws0, expression, do_clause, tag("end"))),
        |t| {
            Node::Loop(Loop {
                kind: LoopKind::Until,
                cond: Box::new(t.2),
                body: Box::new(t.3),
                bindings: None,
            })
        },
    )(i)
}

/// `for` *for_variable* [ no ⏎ ] `in` *expression* *do_clause* `end`
pub(crate) fn for_expression(i: Input) -> NodeResult {
    map(
        tuple((
            tag("for"),
            ws0,
            for_variable,
            no_lt,
            tag("in"),
            ws0,
            expression,
            ws0,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_while_expression() {
        use_parser!(while_expression);
        // Parse errors
        assert_err!("while 1\ndo 2 end");
        assert_err!("while 1do2; 3end");
        // Success cases
        assert_ok!(
            "while 1\n2end",
            Node::loop_(
                LoopKind::While,
                Node::int(1),
                Node::Block(vec![Node::int(2)]),
                vec![]
            )
        );
        assert_ok!(
            "while 1do 2; 3end",
            Node::loop_(
                LoopKind::While,
                Node::int(1),
                Node::Block(vec![Node::int(2), Node::int(3)]),
                vec![]
            )
        );
        assert_ok!(
            "while 1 \n2\n3\nend",
            Node::loop_(
                LoopKind::While,
                Node::int(1),
                Node::Block(vec![Node::int(2), Node::int(3)]),
                vec![]
            )
        );
    }
}
