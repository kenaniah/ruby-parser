use crate::lexers::identifier::{
    class_variable_identifier, constant_identifier, global_variable_identifier,
    instance_variable_identifier, local_variable_identifier,
};
use crate::{Input, Token, TokenResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

pub(crate) fn variable_reference(i: Input) -> TokenResult {
    alt((variable, pseudo_variable))(i)
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
    map(tag("nil"), |_| Token::Nil)(i)
}

pub(crate) fn true_expression(i: Input) -> TokenResult {
    map(tag("true"), |_| Token::True)(i)
}

pub(crate) fn false_expression(i: Input) -> TokenResult {
    map(tag("false"), |_| Token::False)(i)
}

pub(crate) fn self_expression(i: Input) -> TokenResult {
    map(tag("self"), |_| Token::Self_)(i)
}
