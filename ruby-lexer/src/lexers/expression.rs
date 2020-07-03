use crate::{ExpressionResult, Input};

pub fn expression(i: Input) -> ExpressionResult {
    stub(i)
}

fn stub(i: Input) -> ExpressionResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Char)))
}
