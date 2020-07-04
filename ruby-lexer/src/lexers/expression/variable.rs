use crate::lexers::identifier::*;
use crate::{Input, Token, TokenResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, not, peek};
use nom::sequence::tuple;

pub(crate) fn variable_reference(i: Input) -> TokenResult {
    alt((pseudo_variable, variable))(i)
}

pub(crate) fn variable(i: Input) -> TokenResult {
    alt((
        constant_identifier,
        global_variable_identifier,
        class_variable_identifier,
        instance_variable_identifier,
        local_variable_identifier,
    ))(i)
}

pub(crate) fn pseudo_variable(i: Input) -> TokenResult {
    alt((
        nil_expression,
        true_expression,
        false_expression,
        self_expression,
    ))(i)
}

pub(crate) fn nil_expression(i: Input) -> TokenResult {
    map(tuple((tag("nil"), not(peek(identifier_character)))), |_| {
        Token::Nil
    })(i)
}

pub(crate) fn true_expression(i: Input) -> TokenResult {
    map(tuple((tag("true"), not(peek(identifier_character)))), |_| Token::True)(i)
}

pub(crate) fn false_expression(i: Input) -> TokenResult {
    map(tuple((tag("false"), not(peek(identifier_character)))), |_| Token::False)(i)
}

pub(crate) fn self_expression(i: Input) -> TokenResult {
    map(tuple((tag("self"), not(peek(identifier_character)))), |_| Token::Self_)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_reference() {
        use_parser!(variable_reference, Input, Token);
        // Parse errors
        assert_err!("");
        assert_err!("foo ");
        assert_err!("bar\n");
        // Success cases
        assert_ok!("nil", Token::Nil);
        assert_ok!("true", Token::True);
        assert_ok!("false", Token::False);
        assert_ok!("self", Token::Self_);
        assert_ok!("TRUE", Token::ConstantIdentifier("TRUE".to_owned()));
        assert_ok!("False", Token::ConstantIdentifier("False".to_owned()));
        assert_ok!(
            "nil_var",
            Token::LocalVariableIdentifier("nil_var".to_owned())
        );
        assert_ok!("$true", Token::GlobalVariableIdentifier("$true".to_owned()));
    }
}
