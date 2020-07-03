use crate::{Input, StatementResult};
use crate::lexers::expression;
use nom::branch::alt;

/// *expression_statement* | *alias_statement* | *undef_statement* | *if_modifier_statement* | *unless_modifier_statement* | *while_modifier_statement* | *until_modifier_statement* | *rescue_modifier_statement* | *assignment_statement*
pub fn statement(i: Input) -> StatementResult {
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

pub(crate) fn expression_statement(i: Input) -> StatementResult {
    expression(i) // as StatementResult
}

pub(crate) fn alias_statement(i: Input) -> StatementResult {
    stub(i)
}

pub(crate) fn undef_statement(i: Input) -> StatementResult {
    stub(i)
}

pub(crate) fn if_modifier_statement(i: Input) -> StatementResult {
    stub(i)
}

pub(crate) fn unless_modifier_statement(i: Input) -> StatementResult {
    stub(i)
}

pub(crate) fn while_modifier_statement(i: Input) -> StatementResult {
    stub(i)
}

pub(crate) fn until_modifier_statement(i: Input) -> StatementResult {
    stub(i)
}

pub(crate) fn rescue_modifier_statement(i: Input) -> StatementResult {
    stub(i)
}

pub(crate) fn assignment_statement(i: Input) -> StatementResult {
    stub(i)
}

fn stub(i: Input) -> StatementResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Char)))
}
