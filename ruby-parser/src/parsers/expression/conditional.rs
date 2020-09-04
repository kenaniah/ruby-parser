use crate::ast::{Conditional, ConditionalKind};
use crate::lexer::*;
use crate::parsers::expression::expression;
use crate::parsers::program::{compound_statement, separator, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;
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

fn stub(i: Input) -> NodeResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
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
                Node::integer(1),
                Node::Block(vec![Node::integer(2)]),
                Node::empty()
            )
        );
        assert_ok!(
            "if 1; else 2 end",
            Node::conditional(
                ConditionalKind::If,
                Node::integer(1),
                Node::empty(),
                Node::Block(vec![Node::integer(2)]),
            )
        );
        assert_ok!(
            "if 1  and  2 then 3; end",
            Node::conditional(
                ConditionalKind::If,
                Node::logical_and(Node::integer(1), Node::integer(2)),
                Node::Block(vec![Node::integer(3)]),
                Node::empty()
            )
        );
        assert_ok!(
            "if 1 \n2 else 3\n end",
            Node::conditional(
                ConditionalKind::If,
                Node::integer(1),
                Node::Block(vec![Node::integer(2)]),
                Node::Block(vec![Node::integer(3)]),
            )
        );
        assert_ok!(
            "if 1 \n2 elsif 3 then 4 elsif 5\n6 else 7 end",
            Node::conditional(
                ConditionalKind::If,
                Node::integer(1),
                Node::Block(vec![Node::integer(2)]),
                Node::conditional(
                    ConditionalKind::Elsif,
                    Node::integer(3),
                    Node::Block(vec![Node::integer(4)]),
                    Node::conditional(
                        ConditionalKind::Elsif,
                        Node::integer(5),
                        Node::Block(vec![Node::integer(6)]),
                        Node::Block(vec![Node::integer(7)]),
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
                Node::integer(1),
                Node::Block(vec![Node::integer(2)]),
                Node::empty()
            )
        );
        assert_ok!(
            "unless 1 then else 3 end",
            Node::conditional(
                ConditionalKind::Unless,
                Node::integer(1),
                Node::empty(),
                Node::Block(vec![Node::integer(3)])
            )
        );
    }
}
