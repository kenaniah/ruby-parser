use crate::*;

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
    // Literal values
    Literal(Literal),
    // String literals
    String(String),
    InterpolatedString(Vec<Token>),
    ExternalCommand(String),
    InterpolatedExternalCommand(Vec<Token>),
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
    Keyword(String),
    Nil,
    Self_,
    Expression(Expression),
    Block(StatementList),
    InterpolatedSymbol(Vec<Token>),
    Segment(String),
}

impl From<bool> for Token {
    fn from(val: bool) -> Self {
        Self::Literal(Literal::Boolean(val))
    }
}

impl From<isize> for Token {
    fn from(val: isize) -> Self {
        Self::Literal(Literal::Integer(val))
    }
}

impl From<f64> for Token {
    fn from(val: f64) -> Self {
        Self::Literal(Literal::Float(val))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_size() {
        assert_eq!(40, std::mem::size_of::<Token>());
    }
}
