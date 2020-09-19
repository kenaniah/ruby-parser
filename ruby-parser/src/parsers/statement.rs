use crate::ast::{Alias, Conditional, ConditionalKind, Loop, LoopKind, Undef};
use crate::lexer::*;
use crate::parsers::expression::assignment::assignment_statement;
use crate::parsers::expression::expression;
use crate::parsers::expression::logical::keyword_and_expression;
use crate::parsers::expression::logical::keyword_or_expression;
use crate::parsers::expression::method::defined_method_name;
use std::convert::TryFrom;

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
        |(node, ast)| Node::update_placeholder(node, ast),
    )(i)
}

/// *expression*
pub(crate) fn expression_statement(i: Input) -> NodeResult {
    expression(i)
}

/// `alias` *method_name_or_symbol* *method_name_or_symbol*
pub(crate) fn alias_statement(i: Input) -> NodeResult {
    map(
        tuple((
            tag("alias"),
            ws,
            method_name_or_symbol,
            ws,
            method_name_or_symbol,
        )),
        |t| Node::Alias(Alias { to: t.2, from: t.4 }),
    )(i)
}

/// `undef` *undef_list*
pub(crate) fn undef_statement(i: Input) -> NodeResult {
    map(tuple((tag("undef"), ws, undef_list)), |t| {
        Node::Undef(Undef { list: t.2 })
    })(i)
}

/// *method_name_or_symbol* ( [ no ⏎ ] `,` *method_name_or_symbol* )*
pub(crate) fn undef_list(i: Input) -> IdentifierListResult {
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
pub(crate) fn method_name_or_symbol(i: Input) -> IdentifierResult {
    map(preceded(opt(char(':')), defined_method_name), |v| {
        Identifier::try_from(v).unwrap()
    })(i)
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
        |(_, kind, _, expr)| match *kind {
            "if" => Node::Conditional(Conditional {
                kind: ConditionalKind::ModifyingIf,
                cond: Box::new(expr),
                then: Box::new(Node::Placeholder),
                otherwise: Box::new(Node::None),
            }),
            "unless" => Node::Conditional(Conditional {
                kind: ConditionalKind::ModifyingUnless,
                cond: Box::new(expr),
                then: Box::new(Node::Placeholder),
                otherwise: Box::new(Node::None),
            }),
            "while" => Node::Loop(Loop {
                kind: LoopKind::ModifyingWhile,
                cond: Box::new(expr),
                body: Box::new(Node::Placeholder),
                bindings: None,
            }),
            "until" => Node::Loop(Loop {
                kind: LoopKind::ModifyingUntil,
                cond: Box::new(expr),
                body: Box::new(Node::Placeholder),
                bindings: None,
            }),
            _ => unreachable!(),
        },
    )(i)
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
    fn test_alias_statement() {
        use_parser!(alias_statement);
        // Parse error
        assert_err!("alias 1 2");
        assert_err!("alias foo");
        assert_err!("alias foo?? bar");
        assert_err!("alias foo? :bar?!");
        // Success cases
        assert_ok!(
            "alias foo? BAR",
            Node::alias(
                Identifier {
                    name: "foo?".to_owned(),
                    kind: IdentifierKind::Method
                },
                Identifier {
                    name: "BAR".to_owned(),
                    kind: IdentifierKind::Constant
                }
            )
        );
        assert_ok!(
            "alias\n\nfoo\t:bar!",
            Node::alias(
                Identifier {
                    name: "foo".to_owned(),
                    kind: IdentifierKind::LocalVariable
                },
                Identifier {
                    name: "bar!".to_owned(),
                    kind: IdentifierKind::Method
                }
            )
        );
        assert_ok!(
            "alias :sym func_name!",
            Node::alias(
                Identifier {
                    name: "sym".to_owned(),
                    kind: IdentifierKind::LocalVariable
                },
                Identifier {
                    name: "func_name!".to_owned(),
                    kind: IdentifierKind::Method
                }
            )
        );
    }

    #[test]
    fn test_undef_statement() {
        use_parser!(undef_statement);
        // Parse error
        assert_err!("undef 1 2");
        assert_err!("undef");
        assert_err!("undef foo?? bar");
        assert_err!("undef foo? :bar?!");
        // Success cases
        assert_ok!(
            "undef foo?",
            Node::undef(vec![Identifier {
                name: "foo?".to_owned(),
                kind: IdentifierKind::Method
            },])
        );
        assert_ok!(
            "undef \n:bar   , BAZ\t,\n foo!",
            Node::undef(vec![
                Identifier {
                    name: "bar".to_owned(),
                    kind: IdentifierKind::LocalVariable
                },
                Identifier {
                    name: "BAZ".to_owned(),
                    kind: IdentifierKind::Constant
                },
                Identifier {
                    name: "foo!".to_owned(),
                    kind: IdentifierKind::Method
                }
            ])
        );
    }

    #[test]
    fn test_statement() {
        use_parser!(statement);
        // Parse errors
        assert_err!("");
        assert_err!("2 if");
        // Success cases
        assert_ok!(
            "2 if true",
            Node::conditional(
                ConditionalKind::ModifyingIf,
                Node::boolean(true),
                Node::int(2),
                Node::None
            )
        );
        assert_ok!(
            "2 if true unless false",
            Node::conditional(
                ConditionalKind::ModifyingUnless,
                Node::boolean(false),
                Node::conditional(
                    ConditionalKind::ModifyingIf,
                    Node::boolean(true),
                    Node::int(2),
                    Node::None
                ),
                Node::None
            )
        );
        //assert_ok!("undef :hi rescue 3 if false");
        assert_ok!("undef :hi if true rescue 3");
        assert_ok!(
            "1 if 2 unless 3 until 4 if 5 or 6",
            Node::conditional(
                ConditionalKind::ModifyingIf,
                Node::logical_or(Node::int(5), Node::int(6)),
                Node::loop_(
                    LoopKind::ModifyingUntil,
                    Node::int(4),
                    Node::conditional(
                        ConditionalKind::ModifyingUnless,
                        Node::int(3),
                        Node::conditional(
                            ConditionalKind::ModifyingIf,
                            Node::int(2),
                            Node::int(1),
                            Node::None
                        ),
                        Node::None
                    ),
                    vec![]
                ),
                Node::None
            )
        );
    }
}
