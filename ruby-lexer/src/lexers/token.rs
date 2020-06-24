use crate::{Input, TokenResult};

/// *keyword* | *identifier* | *punctuator* | *operator* | *literal*
pub fn token(i: Input) -> TokenResult {
    stub_token(i)
}

fn stub_token(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Complete)))
}
