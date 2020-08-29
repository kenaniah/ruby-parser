use crate::lexer::*;
use crate::parsers::expression::primary_expression;
use nom::branch::alt;

/// *expression_statement* | *alias_statement* | *undef_statement* | *if_modifier_statement* | *unless_modifier_statement* | *while_modifier_statement* | *until_modifier_statement* | *rescue_modifier_statement* | *assignment_statement*
pub(crate) fn statement(i: Input) -> NodeResult {
    alt((
        expression_statement,
        alias_statement,
        undef_statement,
        if_modifier_statement,
        unless_modifier_statement,
        while_modifier_statement,
        until_modifier_statement,
        rescue_modifier_statement,
        assignment_statement,
    ))(i)
}

pub(crate) fn expression_statement(i: Input) -> NodeResult {
    println!("Called with {}", i);
    //expression(i)
    primary_expression(i) // FIXME: This should be expression(i) when left recursion is solved
}

pub(crate) fn alias_statement(i: Input) -> NodeResult {
    stub(i)
}

pub(crate) fn undef_statement(i: Input) -> NodeResult {
    stub(i)
}

pub(crate) fn if_modifier_statement(i: Input) -> NodeResult {
    stub(i)
}

pub(crate) fn unless_modifier_statement(i: Input) -> NodeResult {
    stub(i)
}

pub(crate) fn while_modifier_statement(i: Input) -> NodeResult {
    stub(i)
}

pub(crate) fn until_modifier_statement(i: Input) -> NodeResult {
    stub(i)
}

pub(crate) fn rescue_modifier_statement(i: Input) -> NodeResult {
    stub(i)
}

pub(crate) fn assignment_statement(i: Input) -> NodeResult {
    stub(i)
}

fn stub(i: Input) -> NodeResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}