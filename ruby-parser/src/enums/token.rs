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
    InterpolatedString(Vec<Token>),
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

#[allow(dead_code)]
impl Token {
    /// Creates a token that represents a boolean value
    pub(crate) fn boolean(val: bool) -> Self {
        Self::Literal(Literal::Boolean(val))
    }
    /// Creates a token that represents an integer value
    pub(crate) fn integer(val: isize) -> Self {
        Self::Literal(Literal::Integer(val))
    }
    /// Creates a token that represents a float value
    pub(crate) fn float(val: f64) -> Self {
        Self::Literal(Literal::Float(val))
    }
    /// Creates a token that represents a literal string
    pub(crate) fn literal_string(val: &str) -> Self {
        Self::Literal(Literal::String(val.to_owned()))
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
