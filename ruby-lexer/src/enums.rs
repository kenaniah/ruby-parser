use crate::{Expression, StatementList};

#[derive(Debug, PartialEq)]
pub(crate) enum Interpolated {
    Char(char),
    String(String),
    Expression(Token),
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
    InterpolatedSymbol(Vec<Token>)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_size() {
        assert_eq!(32, std::mem::size_of::<Token>());
    }
}
