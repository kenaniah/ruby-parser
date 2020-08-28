use crate::ast::{IdentifierType, Literal};
use crate::lexer::*;
use crate::parsers::token::identifier::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, not, peek};
use nom::sequence::tuple;

/// *pseudo_variable* | *variable*
pub(crate) fn variable_reference(i: Input) -> NodeResult {
    alt((pseudo_variable, variable))(i)
}

/// *constant_identifier* | *global_variable_identifier* | *class_variable_identifier* | *instance_variable_identifier* | *local_variable_identifier*
pub(crate) fn variable(i: Input) -> NodeResult {
    alt((
        constant_identifier,
        global_variable_identifier,
        class_variable_identifier,
        instance_variable_identifier,
        local_variable_identifier,
    ))(i)
}

/// *nil_expression* | *true_expression* | *false_expression* | *self_expression*
pub(crate) fn pseudo_variable(i: Input) -> NodeResult {
    alt((
        nil_expression,
        true_expression,
        false_expression,
        self_expression,
    ))(i)
}

/// `nil`
pub(crate) fn nil_expression(i: Input) -> NodeResult {
    map(tuple((tag("nil"), not(peek(identifier_character)))), |_| {
        Node::Nil
    })(i)
}

/// `true`
pub(crate) fn true_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("true"), not(peek(identifier_character)))),
        |_| Node::Literal(Literal::Boolean(true)),
    )(i)
}

/// `false`
pub(crate) fn false_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("false"), not(peek(identifier_character)))),
        |_| Node::Literal(Literal::Boolean(false)),
    )(i)
}

/// `self`
pub(crate) fn self_expression(i: Input) -> NodeResult {
    map(
        tuple((tag("self"), not(peek(identifier_character)))),
        |_| Node::Self_,
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_reference() {
        use_parser!(variable_reference);
        // Parse errors
        assert_err!("");
        assert_err!("nil ");
        assert_err!("bar\n");
        // Success cases
        assert_ok!("nil", Node::Nil);
        assert_ok!("true", Node::boolean(true));
        assert_ok!("false", Node::boolean(false));
        assert_ok!("self", Node::Self_);
        assert_ok!("TRUE", Node::ident("TRUE", IdentifierType::Constant));
        assert_ok!("False", Node::ident("False", IdentifierType::Constant));
        assert_ok!("nil_", Node::ident("nil_", IdentifierType::LocalVariable));
        assert_ok!(
            "$true",
            Node::ident("$true", IdentifierType::GlobalVariable)
        );
    }
}
