use crate::ast::{IdentifierType, Literal};
use crate::lexer::{Expression, StatementList};
use crate::Input;

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
    InterpolatedCommand(Vec<Token>),
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
    /// Creates a token that represents an identifier
    pub(crate) fn ident(name: &str, ty: IdentifierType) -> Self {
        match ty {
            IdentifierType::AssignmentMethod => Token::AssignmentMethodIdentifier(name.to_owned()),
            IdentifierType::ClassVariable => Token::ClassVariableIdentifier(name.to_owned()),
            IdentifierType::Constant => Token::ConstantIdentifier(name.to_owned()),
            IdentifierType::GlobalVariable => Token::GlobalVariableIdentifier(name.to_owned()),
            IdentifierType::InstanceVariable => Token::InstanceVariableIdentifier(name.to_owned()),
            IdentifierType::LocalVariable => Token::LocalVariableIdentifier(name.to_owned()),
            IdentifierType::Method => Token::MethodIdentifier(name.to_owned()),
        }
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
