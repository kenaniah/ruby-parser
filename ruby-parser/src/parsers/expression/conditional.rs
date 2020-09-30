use crate::ast::{BinaryOpKind, Case, Conditional, ConditionalKind, WhenClause};
use crate::lexer::*;
use crate::parsers::expression::argument::comma;
use crate::parsers::expression::argument::operator_expression_list;
use crate::parsers::expression::argument::splatting_argument;
use crate::parsers::expression::{expression, operator_expression, range_constructor};
use crate::parsers::program::separator_list;
use crate::parsers::program::{compound_statement, separator};
use std::mem;

/// `if` *expression* *then_clause* *elsif_clause** *else_clause*? `end`
pub(crate) fn if_expression(i: Input) -> NodeResult {
    map(
        tuple((
            tag("if"),
            ws0,
            expression,
            then_clause,
            many0(elsif_clause),
            opt(else_clause),
            tag("end"),
        )),
        |t| {
            let mut parent_node = Node::Conditional(Conditional {
                kind: ConditionalKind::If,
                cond: Box::new(t.2),
                then: Box::new(t.3),
                otherwise: Box::new(t.5.unwrap_or(Node::None)),
            });
            let mut n = &mut parent_node;
            for node in t.4 {
                let mut temp = Box::new(node);
                if let Node::Conditional(a) = n {
                    mem::swap(&mut a.otherwise, &mut temp);
                    if let Node::Conditional(ref mut b) = &mut *a.otherwise {
                        mem::swap(&mut b.otherwise, &mut temp);
                    }
                    n = &mut a.otherwise;
                }
            }
            parent_node
        },
    )(i)
}

/// *separator* *compound_statement* | *separator*? `then` *compound_statement*
pub(crate) fn then_clause(i: Input) -> NodeResult {
    alt((
        map(tuple((separator, compound_statement)), |t| t.1),
        map(
            tuple((opt(separator), ws0, tag("then"), compound_statement)),
            |t| t.3,
        ),
    ))(i)
}

/// `else` *compound_statement*
pub(crate) fn else_clause(i: Input) -> NodeResult {
    map(tuple((tag("else"), compound_statement)), |t| t.1)(i)
}

/// `elsif` *expression* *then_clause*
pub(crate) fn elsif_clause(i: Input) -> NodeResult {
    map(tuple((tag("elsif"), ws0, expression, then_clause)), |t| {
        Node::Conditional(Conditional {
            kind: ConditionalKind::Elsif,
            cond: Box::new(t.2),
            then: Box::new(t.3),
            otherwise: Box::new(Node::None),
        })
    })(i)
}

/// `unless` *expression* *then_clause* *else_clause*? `end`
pub(crate) fn unless_expression(i: Input) -> NodeResult {
    map(
        tuple((
            tag("unless"),
            ws0,
            expression,
            then_clause,
            opt(else_clause),
            tag("end"),
        )),
        |t| {
            Node::Conditional(Conditional {
                kind: ConditionalKind::Unless,
                cond: Box::new(t.2),
                then: Box::new(t.3),
                otherwise: Box::new(t.4.unwrap_or(Node::None)),
            })
        },
    )(i)
}

/// `case` *expression*? *separator_list*? *when_clause*+ *else_clause*? end
pub(crate) fn case_expression(i: Input) -> NodeResult {
    map(
        tuple((
            tag("case"),
            ws0,
            opt(expression),
            ws0,
            opt(separator_list),
            many1(when_clause),
            opt(else_clause),
            tag("end"),
        )),
        |t| {
            Node::Case(Case {
                expr: Box::new(t.2.unwrap_or(Node::None)),
                when: t.5,
                otherwise: Box::new(t.6.unwrap_or(Node::None)),
            })
        },
    )(i)
}

/// `when` *when_argument* *then_clause*
pub(crate) fn when_clause(i: Input) -> WhenClauseResult {
    map(tuple((tag("when"), ws0, when_argument, then_clause)), |t| {
        WhenClause {
            when: t.2,
            then: Box::new(t.3),
        }
    })(i)
}

/// *operator_expression_list* ( [ no ⏎ ] `,`  *splatting_argument* )? | *splatting_argument*
pub(crate) fn when_argument(i: Input) -> NodeListResult {
    alt((
        map(
            tuple((
                operator_expression_list,
                opt(preceded(comma, splatting_argument)),
            )),
            |(mut vec, splat)| {
                if let Some(v) = splat {
                    vec.push(v)
                };
                vec
            },
        ),
        map(splatting_argument, |v| vec![v]),
    ))(i)
}

/// *range_constructor* | *range_constructor* [ no ⏎ ] `?` *operator_expression* [ no ⏎ ] `:` *operator_expression*
pub(crate) fn conditional_operator_expression(i: Input) -> NodeResult {
    let i = stack_frame!("conditional_operator_expression", i);
    map(
        tuple((range_constructor, opt(_conditional_operator_expression))),
        Node::decurse,
    )(i)
}

fn _conditional_operator_expression(i: Input) -> NodeResult {
    let i = stack_frame!("_conditional_operator_expression", i);
    map(
        tuple((
            no_lt,
            char('?'),
            ws0,
            operator_expression,
            no_lt,
            char(':'),
            ws0,
            operator_expression,
            opt(_conditional_operator_expression),
        )),
        |t| {
            let node = Node::Conditional(Conditional {
                cond: Box::new(Node::Placeholder),
                then: Box::new(t.3),
                otherwise: Box::new(t.7),
                kind: ConditionalKind::Ternary,
            });
            Node::decurse((node, t.8))
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_expression() {
        use_parser!(case_expression);
        // Parse errors
        assert_err!("case 1 end");
        assert_err!("case 1 then end");
        // Success cases
        assert_ok!(
            "case when 1, 2 + 3 \n 4 end",
            Node::case(
                Node::None,
                vec![WhenClause {
                    when: vec![
                        Node::int(1),
                        Node::binary_op(Node::int(2), BinaryOpKind::Add, Node::int(3))
                    ],
                    then: Box::new(Node::Block(vec![Node::int(4)]))
                }],
                Node::None
            )
        );
        assert_ok!(
            "case 1 when 2 then 3;when 4, 5 \n 6 when 7 then 8 else 9 end",
            Node::case(
                Node::int(1),
                vec![
                    WhenClause {
                        when: vec![Node::int(2),],
                        then: Box::new(Node::Block(vec![Node::int(3)]))
                    },
                    WhenClause {
                        when: vec![Node::int(4), Node::int(5)],
                        then: Box::new(Node::Block(vec![Node::int(6)]))
                    },
                    WhenClause {
                        when: vec![Node::int(7),],
                        then: Box::new(Node::Block(vec![Node::int(8)]))
                    }
                ],
                Node::Block(vec![Node::int(9)])
            )
        );
    }

    #[test]
    fn test_if_expression() {
        use_parser!(if_expression);
        // Parse errors
        assert_err!("if");
        assert_err!("if 1 end");
        assert_err!("iffoo; 1end");
        // Success cases
        assert_ok!("if@foo; 1end");
        assert_ok!(
            "if 1; 2 end",
            Node::conditional(
                ConditionalKind::If,
                Node::int(1),
                Node::Block(vec![Node::int(2)]),
                Node::None
            )
        );
        assert_ok!(
            "if 1; else 2 end",
            Node::conditional(
                ConditionalKind::If,
                Node::int(1),
                Node::empty(),
                Node::Block(vec![Node::int(2)]),
            )
        );
        assert_ok!(
            "if 1  and  2 then 3; end",
            Node::conditional(
                ConditionalKind::If,
                Node::logical_and(Node::int(1), Node::int(2)),
                Node::Block(vec![Node::int(3)]),
                Node::None
            )
        );
        assert_ok!(
            "if 1 \n2 else 3\n end",
            Node::conditional(
                ConditionalKind::If,
                Node::int(1),
                Node::Block(vec![Node::int(2)]),
                Node::Block(vec![Node::int(3)]),
            )
        );
        assert_ok!(
            "if 1 \n2 elsif 3 then 4 elsif 5\n6 else 7 end",
            Node::conditional(
                ConditionalKind::If,
                Node::int(1),
                Node::Block(vec![Node::int(2)]),
                Node::conditional(
                    ConditionalKind::Elsif,
                    Node::int(3),
                    Node::Block(vec![Node::int(4)]),
                    Node::conditional(
                        ConditionalKind::Elsif,
                        Node::int(5),
                        Node::Block(vec![Node::int(6)]),
                        Node::Block(vec![Node::int(7)]),
                    ),
                ),
            )
        );
    }

    #[test]
    fn test_unless_expression() {
        use_parser!(unless_expression);
        // Parse errors
        assert_err!("if");
        assert_err!("unless 1 end");
        // Success cases
        assert_ok!(
            "unless 1; 2 end",
            Node::conditional(
                ConditionalKind::Unless,
                Node::int(1),
                Node::Block(vec![Node::int(2)]),
                Node::None
            )
        );
        assert_ok!(
            "unless 1 then else 3 end",
            Node::conditional(
                ConditionalKind::Unless,
                Node::int(1),
                Node::empty(),
                Node::Block(vec![Node::int(3)])
            )
        );
    }

    #[test]
    fn test_conditional_operator_expression() {
        use_parser!(conditional_operator_expression);
        // Parse errors
        assert_err!("");
        assert_err!("?2:3");
        assert_err!("1?:3");
        // Success cases
        assert_ok!("\"hi\"", Node::literal_string("hi"));
        let ok = Node::conditional(
            ConditionalKind::Ternary,
            Node::int(1),
            Node::int(2),
            Node::int(3),
        );
        assert_ok!("1 ? 2 : 3", ok);
        assert_ok!("1 ? 2: 3", ok);
        assert_ok!("1?2 : 3", ok);
        assert_ok!("1 ?2 :3", ok);
        assert_ok!("1 ? 2:3", ok);
        assert_ok!("1?2:3", ok);
        assert_ok!(
            "1 ? 2 ? 3 : 4 : 5",
            Node::conditional(
                ConditionalKind::Ternary,
                Node::int(1),
                Node::conditional(
                    ConditionalKind::Ternary,
                    Node::int(2),
                    Node::int(3),
                    Node::int(4)
                ),
                Node::int(5),
            )
        );
        assert_ok!(
            "1??2:?3",
            Node::conditional(
                ConditionalKind::Ternary,
                Node::int(1),
                Node::literal_string("2"),
                Node::literal_string("3"),
            )
        );
    }
}
