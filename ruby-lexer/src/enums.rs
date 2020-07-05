use crate::{Expression, StatementList};

/// Defines a segment of something that may be interpolated
#[derive(Debug, PartialEq)]
pub(crate) enum Segment {
    Char(char),
    String(String),
    Expr(Token),
}

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

/// Internal enum used by the numeric_literal parser
#[derive(Debug, PartialEq)]
pub(crate) enum Numeric {
    Integer(isize),
    Float(f64),
}

/// Defines the tokens that are returned as a result of lexing
#[derive(Debug, PartialEq)]
pub enum Token {
    // Identifiers
    LocalVariableIdentifier(String),
    GlobalVariableIdentifier(String),
    ClassVariableIdentifier(String),
    InstanceVariableIdentifier(String),
    ConstantIdentifier(String),
    MethodIdentifier(String),
    AssignmentMethodIdentifier(String),
    // Numeric literals
    Integer(isize),
    Float(f64),
    // String literals
    SingleQuotedString(String),
    DoubleQuotedString(String),
    InterpolatedString(Vec<Token>),
    // Punctuation
    /// [
    LeftBracket,
    /// ]
    RightBracket,
    /// {
    LeftBrace,
    /// }
    RightBrace,
    /// (
    LeftParen,
    /// )
    RightParen,
    /// ..
    DotDot,
    /// ...
    DotDotDot,
    /// :
    Colon,
    /// ;
    Semicolon,
    /// ::
    DoubleColon,
    /// ,
    Comma,
    /// =>
    Arrow,
    /// ?
    QuestionMark,
    // Program
    LineTerminator,
    Whitespace,
    Operator(String),
    Comment(String),
    EndOfProgram,
    Nil,
    True,
    False,
    Self_,
    Expression(Expression),
    Block(StatementList),
    Symbol(String),
    InterpolatedSymbol(Vec<Token>),
    Segment(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_size() {
        assert_eq!(32, std::mem::size_of::<Token>());
    }
}
