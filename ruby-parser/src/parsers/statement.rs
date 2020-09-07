use crate::ast::{Conditional, ConditionalKind};
use crate::lexer::*;
use crate::parsers::expression::assignment::assignment_statement;
use crate::parsers::expression::expression;
use crate::parsers::token::literal::symbol;

/// *expression_statement* | *alias_statement* | *undef_statement* | *expressionn_modifier_statement* | *rescue_modifier_statement* | *assignment_statement*
pub(crate) fn statement(i: Input) -> NodeResult {
    alt((
        expression_statement,
        alias_statement,
        undef_statement,
        expression_modifier_statement,
        rescue_modifier_statement,
        assignment_statement,
    ))(i)
}

/// *expression*
pub(crate) fn expression_statement(i: Input) -> NodeResult {
    expression(i)
}

/// `alias` *method_name_or_symbol* *method_name_or_symbol*
pub(crate) fn alias_statement(i: Input) -> NodeResult {
    stub(i)
}

/// `undef` *undef_list*
pub(crate) fn undef_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *method_name_or_symbol* ( `,` *method_name_or_symbol* )*
pub(crate) fn undef_list(i: Input) -> NodeResult {
    stub(i)
}

/// *defined_method_name* | *symbol*
pub(crate) fn method_name_or_symbol(i: Input) -> NodeResult {
    //alt((defined_method_name, symbol))(i)
    stub(i)
}

/// *statement* [ no ⏎ ] ( `if` | `unless` | `while` | `until` ) *expression*
pub(crate) fn expression_modifier_statement(i: Input) -> NodeResult {
    stub(i)
    // map(tuple((statement, no_lt, tag("if"), ws, expression)), |t| {
    //     Node::Conditional(Conditional {
    //         kind: ConditionalKind::ModifyingIf,
    //         cond: Box::new(t.4),
    //         then: Box::new(t.0),
    //         otherwise: Box::new(Node::empty()),
    //     })
    // })(i)
}

/// *statement* [ no ⏎ ] `rescue` *fallback_statement*
pub(crate) fn rescue_modifier_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *statement* **but not** *fallback_not_allowed*
pub(crate) fn fallback_statement(i: Input) -> NodeResult {
    stub(i)
}

/// *keyword_and_expression* | *keyword_or_expression* | *if_modifier_statement* | *unless_modifier_statement* | *while_modifier_statement* | *until_modifier_statement* | *rescue_modifier_statement*
pub(crate) fn fallback_not_allowed(i: Input) -> NodeResult {
    stub(i)
}
