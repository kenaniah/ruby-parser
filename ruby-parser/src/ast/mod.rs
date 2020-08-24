//! Provides the abstract syntax tree

mod binary_op;
mod expr;
mod identifier;
mod interpolated;
mod literal;
mod logical;
mod node;
mod unary_op;

use crate::Parsed;

pub use binary_op::*;
pub use expr::*;
pub use identifier::*;
pub use interpolated::*;
pub use literal::*;
pub use logical::*;
pub use node::*;
pub use unary_op::*;

// Describes a parsed AST node
pub(crate) type NodeResult<'a> = Parsed<'a, Node>;

/// Describes a parsed literal
pub(crate) type LiteralResult<'a> = Parsed<'a, Literal>;
