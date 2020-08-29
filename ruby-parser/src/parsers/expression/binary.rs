use crate::ast::{BinaryOp, BinaryOpKind as Op};
use crate::lexer::*;
use crate::parsers::expression::unary::{unary_expression, unary_minus_expression};
use crate::parsers::program::{no_lt, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::{map, opt};
use nom::sequence::tuple;

/// *relational_expression* | *relational_expression* [ no line terminator here ] ( `<=>` | `===` | `==` | `!=` | `=~` | `!~` ) *relational_expression*
pub(crate) fn equality_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                relational_expression,
                no_lt,
                alt((
                    tag("<=>"),
                    tag("==="),
                    tag("=="),
                    tag("!="),
                    tag("=~"),
                    tag("!~"),
                )),
                ws,
                relational_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: match *t.2 {
                        "<=>" => Op::Compare,
                        "===" => Op::CaseEqual,
                        "==" => Op::Equal,
                        "!=" => Op::NotEqual,
                        "=~" => Op::RegexMatch,
                        "!~" => Op::NotRegexMatch,
                        _ => unreachable!(),
                    },
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        relational_expression,
    ))(i)
}

/// *bitwise_or_expression* | *relational_expression* [ no line terminator here ] ( `>=` | `>` | `<=` | `<` ) *bitwise_or_expression*
pub(crate) fn relational_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                relational_expression,
                no_lt,
                alt((tag(">="), tag(">"), tag("<="), tag("<"))),
                ws,
                bitwise_or_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: match *t.2 {
                        ">=" => Op::GreaterEqual,
                        ">" => Op::GreaterThan,
                        "<=" => Op::LessEqual,
                        "<" => Op::LessThan,
                        _ => unreachable!(),
                    },
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        bitwise_or_expression,
    ))(i)
}

/// *bitwise_and_expression* | *bitwise_or_expression* [ no line terminator here ] ( `|` | `^` ) *bitwise_and_expression*
pub(crate) fn bitwise_or_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                bitwise_or_expression,
                no_lt,
                one_of("|^"),
                ws,
                bitwise_and_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: match t.2 {
                        '|' => Op::BitOr,
                        '^' => Op::BitXor,
                        _ => unreachable!(),
                    },
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        bitwise_and_expression,
    ))(i)
}

/// *bitwise_shift_expression* | *bitwise_and_expression* [ no line terminator here ] `&` *bitwise_shift_expression*
pub(crate) fn bitwise_and_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                bitwise_and_expression,
                no_lt,
                char('&'),
                ws,
                bitwise_shift_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: Op::BitAnd,
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        bitwise_shift_expression,
    ))(i)
}

/// *additive_expression* | *bitwise_shift_expression* [ no line terminator here ] ( `<<` | `>>` ) *additive_expression*
pub(crate) fn bitwise_shift_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                bitwise_shift_expression,
                no_lt,
                alt((tag("<<"), tag(">>"))),
                ws,
                additive_expression,
            )),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: match *t.2 {
                        "<<" => Op::ShiftLeft,
                        ">>" => Op::ShiftRight,
                        _ => unreachable!(),
                    },
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        additive_expression,
    ))(i)
}

/// *multiplicative_expression* | *additive_expression* [ no line terminator here ] ( `+` | `-` ) *multiplicative_expression*
pub(crate) fn additive_expression(i: Input) -> NodeResult {
    println!("In additive_expression {}", i);
    map(
        tuple((multiplicative_expression, opt(_additive_expression))),
        finish_node,
    )(i)
}

pub(crate) fn _additive_expression(i: Input) -> NodeResult {
    map(
        tuple((
            no_lt,
            one_of("+-"),
            ws,
            multiplicative_expression,
            opt(_additive_expression),
        )),
        |t| {
            let op = match t.1 {
                '+' => Op::Add,
                '-' => Op::Subtract,
                _ => unreachable!(),
            };
            partial_node(op, t.3, t.4)
        },
    )(i)
}

/// *unary_minus_expression* | *multiplicative_expression* [ no line terminator here ] ( `*` | `/` | `%` ) *unary_minus_expression*
pub(crate) fn multiplicative_expression(i: Input) -> NodeResult {
    println!("In multiplicative_expression {}", i);
    map(
        tuple((unary_minus_expression, opt(_multiplicative_expression))),
        finish_node,
    )(i)
}

fn _multiplicative_expression(i: Input) -> NodeResult {
    map(
        tuple((
            no_lt,
            one_of("*/%"),
            ws,
            unary_minus_expression,
            opt(_multiplicative_expression),
        )),
        |t| {
            let op = match t.1 {
                '*' => Op::Multiply,
                '/' => Op::Divide,
                '%' => Op::Modulus,
                _ => unreachable!(),
            };
            partial_node(op, t.3, t.4)
        },
    )(i)
}

/// Constructs a partial binary op node, using a placeholder for the left hand side
fn partial_node(op: Op, rhs: Node, rest: Option<Node>) -> Node {
    let node = Node::BinaryOp(BinaryOp {
        op,
        lhs: Box::new(Node::Placeholder),
        rhs: Box::new(rhs),
    });
    if let Some(parent_node) = rest {
        replace_nested_lhs_placeholder(parent_node, node)
    } else {
        node
    }
}

/// Completes a partial binary op node (when existing)
fn finish_node(tuple: (Node, Option<Node>)) -> Node {
    let (lhs, ast) = tuple;
    match ast {
        Some(node @ Node::BinaryOp(_)) => replace_nested_lhs_placeholder(node, lhs),
        _ => lhs,
    }
}

/// Recursively travels nested BinaryOp nodes and replaces the last lhs with the given value
fn replace_nested_lhs_placeholder(mut node: Node, value: Node) -> Node {
    use std::borrow::BorrowMut;
    {
        let mut n = &mut node;
        while let Node::BinaryOp(sub) = n {
            n = sub.lhs.borrow_mut();
        }
        *n = value;
    }
    node
}

/// *unary_expression* | *unary_expression* [ no line terminator here ] `**` *power_expression*
pub(crate) fn power_expression(i: Input) -> NodeResult {
    //println!("In power_expression {}", i);
    alt((
        map(
            tuple((unary_expression, no_lt, tag("**"), ws, power_expression)),
            |t| {
                Node::BinaryOp(BinaryOp {
                    op: Op::Power,
                    lhs: Box::new(t.0),
                    rhs: Box::new(t.4),
                })
            },
        ),
        unary_expression,
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_additive_expression() {
        use_parser!(additive_expression);
        // Parse errors
        assert_err!("");
        assert_err!("2 +");
        // Success cases
        assert_ok!(
            "1 + 2",
            Node::binary_op(Node::integer(1), Op::Add, Node::integer(2))
        );
        assert_ok!(
            "1*2",
            Node::binary_op(Node::integer(1), Op::Multiply, Node::integer(2))
        );
        assert_ok!(
            "1 * 2 + 3",
            Node::binary_op(
                Node::binary_op(Node::integer(1), Op::Multiply, Node::integer(2)),
                Op::Add,
                Node::integer(3)
            )
        );
        assert_ok!(
            "1 + 2 * 3 - 4",
            Node::binary_op(
                Node::binary_op(
                    Node::integer(1),
                    Op::Add,
                    Node::binary_op(Node::integer(2), Op::Multiply, Node::integer(3))
                ),
                Op::Subtract,
                Node::integer(4)
            )
        );
    }

    #[test]
    fn test_multiplicative_expression() {
        use_parser!(multiplicative_expression);
        // Parse errors
        assert_err!("");
        assert_err!("nil ");
        // Success cases
        assert_ok!(":hi", Node::literal_symbol(":hi"));
        assert_ok!(
            "12 / 2",
            Node::binary_op(Node::integer(12), Op::Divide, Node::integer(2))
        );
        assert_ok!(
            "\"hi\" * 3.0/4 % 2",
            Node::binary_op(
                Node::binary_op(
                    Node::binary_op(Node::literal_string("hi"), Op::Multiply, Node::float(3.0)),
                    Op::Divide,
                    Node::integer(4)
                ),
                Op::Modulus,
                Node::integer(2)
            )
        );
    }

    #[test]
    fn test_power_expression() {
        use_parser!(power_expression);
        // Parse errors
        assert_err!("");
        assert_err!("nil ");
        assert_err!("3\n** 4");
        assert_err!("3 # comment ** 4");
        // Success cases
        assert_ok!("nil", Node::Nil);
        assert_ok!(
            "3 **\n# comment\n4**-5.2",
            Node::binary_op(
                Node::integer(3),
                Op::Power,
                Node::binary_op(Node::integer(4), Op::Power, Node::float(-5.2))
            )
        );
    }
}
