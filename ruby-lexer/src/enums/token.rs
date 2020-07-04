use crate::{Expression, StatementList};

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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_size() {
        assert_eq!(32, std::mem::size_of::<Token>());
    }
}
