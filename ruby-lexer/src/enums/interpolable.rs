use crate::{Segment, Token};

/// Defines something that may be interpolated
#[derive(Debug, PartialEq)]
pub enum Interpolatable {
    String(String),
    Interpolated(Vec<Token>),
}

impl From<Vec<Segment>> for Interpolatable {
    fn from(item: Vec<Segment>) -> Self {
        let mut tokens: Vec<Token> = vec![];
        let mut string = String::new();
        let mut interpolated = false;
        for part in item {
            match part {
                Segment::Char(c) => string.push(c),
                Segment::String(s) => string.push_str(&s),
                Segment::Expr(t) => {
                    if !string.is_empty() {
                        tokens.push(Token::Segment(string.clone()));
                        string.clear();
                    }
                    tokens.push(t);
                    interpolated = true;
                }
            }
        }
        if interpolated {
            if !string.is_empty() {
                tokens.push(Token::Segment(string.clone()));
            }
            Self::Interpolated(tokens)
        } else {
            Self::String(string)
        }
    }
}

impl Interpolatable {
    /// Strips leading indentation from the content according to the rules for squiggly heredocs
    pub fn to_unindented(self) -> Self {
        unimplemented!()
    }
}
