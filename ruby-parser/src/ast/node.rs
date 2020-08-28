use super::*;
use crate::lexer::Token;

pub enum Node {
    LogicalAnd(LogicalAnd),
    LogicalOr(LogicalOr),
    LogicalNot(LogicalNot),
    Literal(Literal),
    Identifier(Identifier),
    Interpolated(Interpolated),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Junk(Token),
    FromTokenError,
}

impl From<Token> for Node {
    fn from(token: Token) -> Self {
        match token {
            // Identifiers
            Token::LocalVariableIdentifier(name) => Self::Identifier(Identifier {
                name,
                ty: IdentifierType::LocalVariable,
            }),
            Token::GlobalVariableIdentifier(name) => Self::Identifier(Identifier {
                name,
                ty: IdentifierType::GlobalVariable,
            }),
            Token::ClassVariableIdentifier(name) => Self::Identifier(Identifier {
                name,
                ty: IdentifierType::ClassVariable,
            }),
            Token::InstanceVariableIdentifier(name) => Self::Identifier(Identifier {
                name,
                ty: IdentifierType::InstanceVariable,
            }),
            Token::ConstantIdentifier(name) => Self::Identifier(Identifier {
                name,
                ty: IdentifierType::Constant,
            }),
            Token::MethodIdentifier(name) => Self::Identifier(Identifier {
                name,
                ty: IdentifierType::Method,
            }),
            Token::AssignmentMethodIdentifier(name) => Self::Identifier(Identifier {
                name,
                ty: IdentifierType::AssignmentMethod,
            }),
            // Literals
            Token::Literal(val) => Self::Literal(val),
            Token::Nil => Self::Literal(Literal::Nil),
            // Interpolations
            Token::InterpolatedCommand(val) => Self::Interpolated(Interpolated::Command(val)),
            Token::InterpolatedString(val) => Self::Interpolated(Interpolated::String(val)),
            Token::InterpolatedSymbol(val) => Self::Interpolated(Interpolated::Symbol(val)),
            // Errors
            v => Self::Junk(v),
        }
    }
}
