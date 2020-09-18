use crate::ast::{Conditional, ConditionalKind};
use crate::lexer::*;
use crate::parsers::expression::assignment::assignment_statement;
use crate::parsers::expression::expression;
use crate::parsers::expression::logical::keyword_and_expression;
use crate::parsers::expression::logical::keyword_or_expression;
use crate::parsers::expression::method::defined_method_name;
use crate::parsers::token::literal::symbol;

/// *expression_statement* | *alias_statement* | *undef_statement* | *expression_modifier_statement* | *rescue_modifier_statement* | *assignment_statement*
pub(crate) fn statement(i: Input) -> NodeResult {
    map(
        tuple((_simple_statement, opt(_statement_modifier))),
        |(node, ast)| Node::update_placeholder(node, ast),
    )(i)
}

pub(crate) fn _simple_statement(i: Input) -> NodeResult {
    alt((
        expression_statement,
        alias_statement,
        undef_statement,
        assignment_statement,
    ))(i)
}

pub(crate) fn _statement_modifier(i: Input) -> NodeResult {
    map(
        tuple((
            alt((_expression_modifier_statement, _rescue_modifier_statement)),
            opt(_statement_modifier),
        )),
        |t| Node::Placeholder,
    )(i)
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
    map(tuple((tag("undef"), ws, undef_list)), |t| Node::Placeholder)(i)
}

/// *method_name_or_symbol* ( [ no ⏎ ] `,` *method_name_or_symbol* )*
pub(crate) fn undef_list(i: Input) -> NodeListResult {
    map(
        tuple((
            method_name_or_symbol,
            many0(map(
                tuple((no_lt, char(','), ws, method_name_or_symbol)),
                |t| t.3,
            )),
        )),
        |t| {
            let mut vec = vec![t.0];
            vec.extend(t.1);
            vec
        },
    )(i)
}

/// *defined_method_name* | *symbol*
pub(crate) fn method_name_or_symbol(i: Input) -> NodeResult {
    alt((defined_method_name, symbol))(i)
}

/// *statement* [ no ⏎ ] ( `if` | `unless` | `while` | `until` ) *expression*
pub(crate) fn _expression_modifier_statement(i: Input) -> NodeResult {
    map(
        tuple((
            no_lt,
            alt((tag("if"), tag("unless"), tag("while"), tag("until"))),
            ws,
            expression,
        )),
        |t| Node::Placeholder,
    )(i)
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
pub(crate) fn _rescue_modifier_statement(i: Input) -> NodeResult {
    map(tuple((no_lt, tag("rescue"), ws, fallback_statement)), |t| {
        Node::Placeholder
    })(i)
}

/// *statement* **but not** *fallback_not_allowed*
pub(crate) fn fallback_statement(i: Input) -> NodeResult {
    let (i, _) = peek(not(fallback_not_allowed))(i)?;
    statement(i)
}

/// *keyword_and_expression* | *keyword_or_expression* | *if_modifier_statement* | *unless_modifier_statement* | *while_modifier_statement* | *until_modifier_statement* | *rescue_modifier_statement*
pub(crate) fn fallback_not_allowed(i: Input) -> LexResult {
    alt((
        recognize(keyword_and_expression),
        recognize(keyword_or_expression),
        recognize(tuple((_simple_statement, _statement_modifier))),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statement() {
        use_parser!(statement);
        // Parse errors
        assert_err!("");
        assert_err!("2 if");
        // Success cases
        assert_ok!("2 if true");
        //assert_ok!("undef :hi rescue 3 if false");
        assert_ok!("undef :hi if true rescue 3");
        assert_ok!("1 if 2 unless 3 until 4 if 5 or 6");
    }
}
