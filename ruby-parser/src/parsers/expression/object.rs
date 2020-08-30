use crate::ast::Ranged;
use crate::lexer::*;
use crate::parsers::expression::logical::operator_or_expression;
use crate::parsers::expression::operator_expression;
use crate::parsers::program::{no_lt, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::sequence::tuple;

/// `[` *indexing_argument_list*? `]`
pub(crate) fn array_constructor(i: Input) -> NodeResult {
    stub(i)
}

/// `{` ( *association_list* [ no line terminator here ] `,`? )? `}`
pub(crate) fn hash_constructor(i: Input) -> NodeResult {
    stub(i)
}

/// *association* ( [ no line terminator here ] `,` *association* )*
pub(crate) fn association_list(i: Input) -> NodeResult {
    stub(i)
}

/// *association_key* [ no line terminator here ] `=>` *association_value*
pub(crate) fn association(i: Input) -> NodeResult {
    stub(i)
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
    //
    //     |t| {
    //         Node::Ranged(Ranged {
    //             from: Box::new(expr),
    //             to: Box::new(t.3),
    //             exclusive: *t.1 == "...",
    //         })
    //     },
    // )(i){
    //
    // }else{
    //     node
    // }
}

/// `..` | `...`
pub(crate) fn range_operator(i: Input) -> LexResult {
    recognize(alt((tag("..."), tag(".."))))(i)
}

fn stub(i: Input) -> NodeResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}

#[cfg(test)]
mod tests {
    use super::*;

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
