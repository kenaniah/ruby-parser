use crate::{Segment, Token};
use std::cmp::min;

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
        match self {
            Self::Interpolated(tokens) => Self::Interpolated(Self::unindent_tokens(tokens)),
            Self::String(string) => {
                let mut tokens = Self::unindent_tokens(vec![Token::Segment(string)]);
                if let Token::Segment(string) = tokens.remove(0) {
                    Self::String(string)
                } else {
                    unreachable!()
                }
            }
        }
    }
    fn unindent_tokens(mut tokens: Vec<Token>) -> Vec<Token> {
        let mut after_newline = true;
        let mut indentation = usize::MAX;
        // Determine the indentation level
        for t in &tokens {
            if let Token::Segment(string) = t {
                for line in string.lines() {
                    let mut whitespace = 0usize;
                    if after_newline {
                        for c in line.chars() {
                            match c {
                                ' ' | '\t' => whitespace += 1,
                                _ => {
                                    // Short-circuit if no adjustments are needed
                                    if whitespace == 0 {
                                        return tokens;
                                    }
                                    indentation = min(indentation, whitespace);
                                    break;
                                }
                            }
                        }
                    };
                    after_newline = true;
                }
            } else {
                after_newline = false;
            }
        }
        // Return if no adjustments need to be made
        if indentation == usize::MAX {
            return tokens;
        }
        // Adjust the indentation of string segments accordingly
        after_newline = true;
        let mut whitespace = indentation;
        for t in &mut tokens {
            if let Token::Segment(ref mut string) = t {
                let mut new_string = String::new();
                for c in string.chars() {
                    match c {
                        ' ' | '\t' => {
                            if after_newline && whitespace > 0 {
                                whitespace -= 1;
                                continue;
                            }
                            new_string.push(c);
                        }
                        '\n' => {
                            after_newline = true;
                            whitespace = indentation;
                            new_string.push(c);
                        }
                        _ => {
                            after_newline = false;
                            new_string.push(c);
                        }
                    };
                }
                *string = new_string;
            } else {
                after_newline = false;
            }
        }
        tokens
    }
}
