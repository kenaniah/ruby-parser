use crate::ast::{Conditional, ConditionalKind};
use crate::lexer::*;
use crate::parsers::expression::{expression, operator_expression, range_constructor};
use crate::parsers::program::{compound_statement, separator};
use std::mem;

/// `if` *expression* *then_clause* *elsif_clause** *else_clause*? `end`
pub(crate) fn if_expression(i: Input) -> NodeResult {
    map(
        tuple((
            tag("if"),
            ws,
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
                otherwise: Box::new(t.5.unwrap_or(Node::empty())),
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
        map(tuple((separator, ws, compound_statement, ws)), |t| t.2),
        map(
            tuple((opt(separator), ws, tag("then"), ws, compound_statement, ws)),
            |t| t.4,
        ),
    ))(i)
}

/// `else` *compound_statement*
pub(crate) fn else_clause(i: Input) -> NodeResult {
    map(tuple((tag("else"), ws, compound_statement, ws)), |t| t.2)(i)
}

/// `elsif` *expression* *then_clause*
pub(crate) fn elsif_clause(i: Input) -> NodeResult {
    map(tuple((tag("elsif"), ws, expression, then_clause)), |t| {
        Node::Conditional(Conditional {
            kind: ConditionalKind::Elsif,
            cond: Box::new(t.2),
            then: Box::new(t.3),
            otherwise: Box::new(Node::empty()),
        })
    })(i)
}

/// `unless` *expression* *then_clause* *else_clause*? `end`
pub(crate) fn unless_expression(i: Input) -> NodeResult {
    map(
        tuple((
            tag("unless"),
            ws,
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
                otherwise: Box::new(t.4.unwrap_or(Node::empty())),
            })
        },
    )(i)
}

/// `case` *expression*? *separator_list*? *when_clause*+ *else_clause*? end
pub(crate) fn case_expression(i: Input) -> NodeResult {
    stub(i)
}

/// `when` *when_argument* *then_clause*
pub(crate) fn when_clause(i: Input) -> NodeResult {
    stub(i)
}

/// *operator_expression_list* ( [ no ⏎ ] `,`  *splatting_argument* )? | *splatting_argument*
pub(crate) fn when_argument(i: Input) -> NodeResult {
    stub(i)
}

/// *range_constructor* | *range_constructor* [ no ⏎ ] `?` *operator_expression* [ no ⏎ ] `:` *operator_expression*
pub(crate) fn conditional_operator_expression(i: Input) -> NodeResult {
    let i = stack_frame!("conditional_operator_expression", i);
    map(
        tuple((range_constructor, opt(_conditional_operator_expression))), // BUG: range constructor is currently too greedy
        |(node, ast)| Node::update_placeholder(node, ast),
    )(i)
}

fn _conditional_operator_expression(i: Input) -> NodeResult {
    let i = stack_frame!("_conditional_operator_expression", i);
    map(
        tuple((
            no_lt,
            char('?'),
            ws,
            operator_expression,
            no_lt,
            char(':'),
            ws,
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
            Node::update_placeholder(node, t.8)
        },
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_expression() {
        use_parser!(if_expression);
        // Parse errors
        assert_err!("if");
        assert_err!("if 1 end");
        // Success cases
        assert_ok!(
            "if 1; 2 end",
            Node::conditional(
                ConditionalKind::If,
                Node::int(1),
                Node::Block(vec![Node::int(2)]),
                Node::empty()
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
                Node::empty()
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
                Node::empty()
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
