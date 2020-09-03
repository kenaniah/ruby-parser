use crate::lexer::*;
use crate::parsers::expression::expression;
use crate::parsers::program::{compound_statement, separator, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::tuple;

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
        |t| Node::Placeholder,
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
        Node::Placeholder
    })(i)
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
        assert_ok!("if 1; 2 end", Node::Placeholder);
        assert_ok!("if 1 then 2; end", Node::Placeholder);
        assert_ok!("if 1 \n2 else 3\n end", Node::Placeholder);
        assert_ok!("if 1 \n2 elsif 3 then 4 elsif 5\n6 end", Node::Placeholder);
    }
}
