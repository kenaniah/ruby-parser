use crate::ast::Ranged;
use crate::lexer::*;
use crate::parsers::expression::argument::comma;
use crate::parsers::expression::argument::indexing_argument_list;
use crate::parsers::expression::logical::operator_or_expression;
use crate::parsers::expression::operator_expression;
use crate::parsers::program::{no_lt, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, opt, recognize};
use nom::multi::many0;
use nom::sequence::tuple;

/// `[` *indexing_argument_list*? `]`
pub(crate) fn array_constructor(i: Input) -> NodeResult {
    map(
        tuple((char('['), ws, opt(indexing_argument_list), ws, char(']'))),
        |t| Node::Array(t.2.unwrap_or(vec![])),
    )(i)
}

/// `{` ( *association_list* [ no line terminator here ] `,`? )? `}`
pub(crate) fn hash_constructor(i: Input) -> NodeResult {
    map(
        tuple((
            char('{'),
            ws,
            opt(map(tuple((association_list, opt(comma), ws)), |t| t.0)),
            ws,
            char('}'),
        )),
        |t| Node::Hash(t.2.unwrap_or(vec![])),
    )(i)
}

/// *association* ( [ no line terminator here ] `,` *association* )*
pub(crate) fn association_list(i: Input) -> NodeListResult {
    map(
        tuple((
            association,
            many0(map(tuple((no_lt, char(','), ws, association)), |t| t.3)),
        )),
        |(mut first, vec)| {
            first.extend(vec.into_iter().flatten().collect::<Vec<Node>>());
            first
        },
    )(i)
}

/// *association_key* [ no line terminator here ] `=>` *association_value*
pub(crate) fn association(i: Input) -> NodeListResult {
    map(
        tuple((association_key, no_lt, tag("=>"), ws, association_value)),
        |t| vec![t.0, t.4],
    )(i)
}

/// *operator_expression*
pub(crate) fn association_key(i: Input) -> NodeResult {
    operator_expression(i)
}

/// *operator_expression*
pub(crate) fn association_value(i: Input) -> NodeResult {
    operator_expression(i)
}

/// *operator_or_expression* | *operator_or_expression* [ no line terminator here ] *range_operator* *operator_or_expression*
pub(crate) fn range_constructor(i: Input) -> NodeResult {
    let i = stack_frame!("range_constructor", i);
    let (i, lhs) = operator_or_expression(i)?;
    if let Ok((j, t)) = tuple((no_lt, range_operator, ws, operator_or_expression))(i.clone()) {
        Ok((
            j,
            Node::Ranged(Ranged {
                from: Box::new(lhs),
                to: Box::new(t.3),
                exclusive: *t.1 == "...",
            }),
        ))
    } else {
        Ok((i, lhs))
    }
}

/// `..` | `...`
pub(crate) fn range_operator(i: Input) -> LexResult {
    recognize(alt((tag("..."), tag(".."))))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOpKind;

    #[test]
    fn test_hash_constructor() {
        use_parser!(hash_constructor);
        // Parse errors
        assert_err!("{");
        assert_err!("{1 => }");
        assert_err!("{1 \n => 2}");
        // Success cases
        assert_ok!("{}", Node::Hash(vec![]));
        assert_ok!("{1=>2}", Node::Hash(vec![Node::integer(1), Node::integer(2)]));
        assert_ok!(
            "{1 => 2,\n\n 3=>4}",
            Node::Hash(vec![
                Node::integer(1),
                Node::integer(2),
                Node::integer(3),
                Node::integer(4)
            ])
        );
    }

    #[test]
    fn test_array_constructor() {
        use_parser!(array_constructor);
        // Parse errors
        assert_err!("[(]");
        // Success cases
        assert_ok!("[ \n]", Node::array(vec![]));
        assert_ok!(
            "[1, 2 * 3, []]",
            Node::array(vec![
                Node::integer(1),
                Node::binary_op(Node::integer(2), BinaryOpKind::Multiply, Node::integer(3)),
                Node::array(vec![])
            ])
        );
    }

    #[test]
    fn test_range_constructor() {
        use_parser!(range_constructor);
        // Parse errors
        assert_err!("");
        assert_err!("1 ");
        assert_err!("1....5");
        // Success cases
        assert_ok!("2", Node::integer(2));
        assert_ok!(
            "2 ..  5",
            Node::range(Node::integer(2), Node::integer(5), false)
        );
        assert_ok!(
            "2.0...4.0",
            Node::range(Node::float(2.0), Node::float(4.0), true)
        );
    }
}
