use crate::ast::{BinaryOp, BinaryOpKind as Op};
use crate::lexer::*;
use crate::parsers::expression::unary::{unary_expression, unary_minus_expression};

/// *relational_expression* | *relational_expression* [ no ⏎ ] ( `<=>` | `===` | `==` | `!=` | `=~` | `!~` ) *relational_expression*
pub(crate) fn equality_expression(i: Input) -> NodeResult {
    let i = stack_frame!("equality_expression", i);
    map(
        tuple((relational_expression, opt(_equality_expression))),
        |(node, ast)| Node::update_placeholder(node, ast),
    )(i)
}

fn _equality_expression(i: Input) -> NodeResult {
    map(
        tuple((
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
            opt(_equality_expression),
        )),
        |t| {
            let op = match *t.1 {
                "<=>" => Op::Compare,
                "===" => Op::CaseEqual,
                "==" => Op::Equal,
                "!=" => Op::NotEqual,
                "=~" => Op::RegexMatch,
                "!~" => Op::NotRegexMatch,
                _ => unreachable!(),
            };
            _partial_node(op, t.3, t.4)
        },
    )(i)
}

/// *bitwise_or_expression* | *relational_expression* [ no ⏎ ] ( `>=` | `>` | `<=` | `<` ) *bitwise_or_expression*
pub(crate) fn relational_expression(i: Input) -> NodeResult {
    let i = stack_frame!("relational_expression", i);
    map(
        tuple((bitwise_or_expression, opt(_relational_expression))),
        |(node, ast)| Node::update_placeholder(node, ast),
    )(i)
}

fn _relational_expression(i: Input) -> NodeResult {
    map(
        tuple((
            no_lt,
            alt((tag(">="), tag(">"), tag("<="), tag("<"))),
            ws,
            bitwise_or_expression,
            opt(_relational_expression),
        )),
        |t| {
            let op = match *t.1 {
                ">=" => Op::GreaterEqual,
                ">" => Op::GreaterThan,
                "<=" => Op::LessEqual,
                "<" => Op::LessThan,
                _ => unreachable!(),
            };
            _partial_node(op, t.3, t.4)
        },
    )(i)
}

/// *bitwise_and_expression* | *bitwise_or_expression* [ no ⏎ ] ( `|` | `^` ) *bitwise_and_expression*
pub(crate) fn bitwise_or_expression(i: Input) -> NodeResult {
    let i = stack_frame!("bitwise_or_expression", i);
    map(
        tuple((bitwise_and_expression, opt(_bitwise_or_expression))),
        |(node, ast)| Node::update_placeholder(node, ast),
    )(i)
}

fn _bitwise_or_expression(i: Input) -> NodeResult {
    map(
        tuple((
            no_lt,
            one_of("|^"),
            ws,
            bitwise_and_expression,
            opt(_bitwise_or_expression),
        )),
        |t| {
            let op = match t.1 {
                '|' => Op::BitOr,
                '^' => Op::BitXor,
                _ => unreachable!(),
            };
            _partial_node(op, t.3, t.4)
        },
    )(i)
}

/// *bitwise_shift_expression* | *bitwise_and_expression* [ no ⏎ ] `&` *bitwise_shift_expression*
pub(crate) fn bitwise_and_expression(i: Input) -> NodeResult {
    let i = stack_frame!("bitwise_and_expression", i);
    map(
        tuple((bitwise_shift_expression, opt(_bitwise_and_expression))),
        |(node, ast)| Node::update_placeholder(node, ast),
    )(i)
}

fn _bitwise_and_expression(i: Input) -> NodeResult {
    map(
        tuple((
            no_lt,
            char('&'),
            ws,
            bitwise_shift_expression,
            opt(_bitwise_and_expression),
        )),
        |t| _partial_node(Op::BitAnd, t.3, t.4),
    )(i)
}

/// *additive_expression* | *bitwise_shift_expression* [ no ⏎ ] ( `<<` | `>>` ) *additive_expression*
pub(crate) fn bitwise_shift_expression(i: Input) -> NodeResult {
    let i = stack_frame!("bitwise_shift_expression", i);
    map(
        tuple((additive_expression, opt(_bitwise_shift_expression))),
        |(node, ast)| Node::update_placeholder(node, ast),
    )(i)
}

fn _bitwise_shift_expression(i: Input) -> NodeResult {
    map(
        tuple((
            no_lt,
            alt((tag("<<"), tag(">>"))),
            ws,
            additive_expression,
            opt(_bitwise_shift_expression),
        )),
        |t| {
            let op = match *t.1 {
                "<<" => Op::ShiftLeft,
                ">>" => Op::ShiftRight,
                _ => unreachable!(),
            };
            _partial_node(op, t.3, t.4)
        },
    )(i)
}

/// *multiplicative_expression* | *additive_expression* [ no ⏎ ] ( `+` | `-` ) *multiplicative_expression*
pub(crate) fn additive_expression(i: Input) -> NodeResult {
    let i = stack_frame!("additive_expression", i);
    map(
        tuple((multiplicative_expression, opt(_additive_expression))),
        |(node, ast)| Node::update_placeholder(node, ast),
    )(i)
}

fn _additive_expression(i: Input) -> NodeResult {
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
            _partial_node(op, t.3, t.4)
        },
    )(i)
}

/// *unary_minus_expression* | *multiplicative_expression* [ no ⏎ ] ( `*` | `/` | `%` ) *unary_minus_expression*
pub(crate) fn multiplicative_expression(i: Input) -> NodeResult {
    let i = stack_frame!("multiplicative_expression", i);
    map(
        tuple((unary_minus_expression, opt(_multiplicative_expression))),
        |(node, ast)| Node::update_placeholder(node, ast),
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
            _partial_node(op, t.3, t.4)
        },
    )(i)
}

/// *unary_expression* | *unary_expression* [ no ⏎ ] `**` *power_expression*
pub(crate) fn power_expression(i: Input) -> NodeResult {
    let i = stack_frame!("power_expression", i);
    let (i, lhs) = unary_expression(i)?;
    if let Ok((j, t)) = tuple((no_lt, tag("**"), ws, power_expression))(i.clone()) {
        Ok((
            j,
            Node::BinaryOp(BinaryOp {
                op: Op::Power,
                lhs: Box::new(lhs),
                rhs: Box::new(t.3),
            }),
        ))
    } else {
        Ok((i, lhs))
    }
}

/// Constructs a partial binary op node, using a placeholder for the left hand side
fn _partial_node(op: Op, rhs: Node, ast: Option<Node>) -> Node {
    let node = Node::BinaryOp(BinaryOp {
        op,
        lhs: Box::new(Node::Placeholder),
        rhs: Box::new(rhs),
    });
    Node::update_placeholder(node, ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality_expression() {
        use_parser!(equality_expression);
        // Parse errors
        assert_err!("");
        // Success cases
        assert_ok!(
            "1 === 2 + 3",
            Node::binary_op(
                Node::int(1),
                Op::CaseEqual,
                Node::binary_op(Node::int(2), Op::Add, Node::int(3)),
            )
        );
        assert_ok!(
            ":hi != \n\n hello",
            Node::binary_op(
                Node::literal_symbol("hi"),
                Op::NotEqual,
                Node::ident("hello", IdentifierKind::LocalVariable)
            )
        );
        assert_ok!(
            "(1 == 2)",
            Node::Block(vec![
                Node::binary_op(Node::int(1), Op::Equal, Node::int(2),)
            ])
        );
        assert_remaining!("1?2:3", "?2:3");
    }

    #[test]
    fn test_relational_expression() {
        use_parser!(relational_expression);
        // Parse errors
        assert_err!("");
        assert_err!("2 <");
        // Success cases
        assert_ok!(
            "1 >= 2 < 3 + 4 > 5 <= 6",
            Node::binary_op(
                Node::binary_op(
                    Node::binary_op(
                        Node::binary_op(Node::int(1), Op::GreaterEqual, Node::int(2)),
                        Op::LessThan,
                        Node::binary_op(Node::int(3), Op::Add, Node::int(4))
                    ),
                    Op::GreaterThan,
                    Node::int(5)
                ),
                Op::LessEqual,
                Node::int(6)
            )
        );
        assert_remaining!("1?2:3", "?2:3");
    }

    #[test]
    fn test_bitwise_or_expression() {
        use_parser!(bitwise_or_expression);
        // Parse errors
        assert_err!("");
        assert_err!("2 |");
        // Success cases
        assert_ok!(
            "1 | 2 + 3 ^ 4 | 5",
            Node::binary_op(
                Node::binary_op(
                    Node::binary_op(
                        Node::int(1),
                        Op::BitOr,
                        Node::binary_op(Node::int(2), Op::Add, Node::int(3))
                    ),
                    Op::BitXor,
                    Node::int(4)
                ),
                Op::BitOr,
                Node::int(5)
            )
        );
        assert_remaining!("1?2:3", "?2:3");
    }

    #[test]
    fn test_bitwise_and_expression() {
        use_parser!(bitwise_and_expression);
        // Parse errors
        assert_err!("");
        assert_err!("2 && 3");
        // Success cases
        assert_ok!(
            "1 & 2",
            Node::binary_op(Node::int(1), Op::BitAnd, Node::int(2))
        );
        assert_ok!(
            "1+2&3",
            Node::binary_op(
                Node::binary_op(Node::int(1), Op::Add, Node::int(2)),
                Op::BitAnd,
                Node::int(3)
            )
        );
        assert_remaining!("1?2:3", "?2:3");
    }

    #[test]
    fn test_bitwise_shift_expression() {
        use_parser!(bitwise_shift_expression);
        // Parse errors
        assert_err!("");
        assert_err!("2 << << 4");
        // Success cases
        assert_ok!(
            "1 << 2",
            Node::binary_op(Node::int(1), Op::ShiftLeft, Node::int(2))
        );
        assert_ok!(
            "1 - 2 >> 3",
            Node::binary_op(
                Node::binary_op(Node::int(1), Op::Subtract, Node::int(2)),
                Op::ShiftRight,
                Node::int(3)
            )
        );
        assert_ok!(
            "1 << 2 * -3",
            Node::binary_op(
                Node::int(1),
                Op::ShiftLeft,
                Node::binary_op(Node::int(2), Op::Multiply, Node::int(-3)),
            )
        );
        assert_remaining!("1?2:3", "?2:3");
    }

    #[test]
    fn test_additive_expression() {
        use_parser!(additive_expression);
        // Parse errors
        assert_err!("");
        assert_err!("2 +");
        // Success cases
        assert_ok!("1+ 2", Node::binary_op(Node::int(1), Op::Add, Node::int(2)));
        assert_ok!(
            "1 - 2 -3",
            Node::binary_op(
                Node::binary_op(Node::int(1), Op::Subtract, Node::int(2)),
                Op::Subtract,
                Node::int(3)
            )
        );
        assert_ok!(
            "1*2",
            Node::binary_op(Node::int(1), Op::Multiply, Node::int(2))
        );
        assert_ok!(
            "1 * 2 + 3",
            Node::binary_op(
                Node::binary_op(Node::int(1), Op::Multiply, Node::int(2)),
                Op::Add,
                Node::int(3)
            )
        );
        assert_ok!(
            "1 + 2 * 3 - 4",
            Node::binary_op(
                Node::binary_op(
                    Node::int(1),
                    Op::Add,
                    Node::binary_op(Node::int(2), Op::Multiply, Node::int(3))
                ),
                Op::Subtract,
                Node::int(4)
            )
        );
        assert_remaining!("1?2:3", "?2:3");
    }

    #[test]
    fn test_multiplicative_expression() {
        use_parser!(multiplicative_expression);
        // Parse errors
        assert_err!("");
        assert_err!("nil ");
        // Success cases
        assert_ok!(":hi", Node::literal_symbol("hi"));
        assert_ok!(
            "12 / 2",
            Node::binary_op(Node::int(12), Op::Divide, Node::int(2))
        );
        assert_ok!(
            "\"hi\" * 3.0/4 % 2",
            Node::binary_op(
                Node::binary_op(
                    Node::binary_op(Node::literal_string("hi"), Op::Multiply, Node::float(3.0)),
                    Op::Divide,
                    Node::int(4)
                ),
                Op::Modulus,
                Node::int(2)
            )
        );
        assert_remaining!("1?2:3", "?2:3");
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
                Node::int(3),
                Op::Power,
                Node::binary_op(Node::int(4), Op::Power, Node::float(-5.2))
            )
        );
        assert_remaining!("1?2:3", "?2:3");
    }
}
