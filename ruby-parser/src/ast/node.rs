use super::*;
use crate::Token;

pub enum Node {
    Literal(Literal),
    Identifier(Identifier),
    Interpolated(Interpolated),
    BinaryOp(BinaryOp),
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
            Token::String(val) => Self::Literal(Literal::String(val)),
            Token::ExternalCommand(val) => Self::Literal(Literal::ExternalCommand(val)),
            Token::Nil => Self::Literal(Literal::Nil),
            Token::True => Self::Literal(Literal::Boolean(true)),
            Token::False => Self::Literal(Literal::Boolean(false)),
            // Interpolations
            Token::InterpolatedExternalCommand(val) => {
                Self::Interpolated(Interpolated::ExternalCommand(val))
            }
            Token::InterpolatedString(val) => Self::Interpolated(Interpolated::String(val)),
            Token::InterpolatedSymbol(val) => Self::Interpolated(Interpolated::Symbol(val)),
            // Errors
            _ => Self::FromTokenError,
        }
    }
}
