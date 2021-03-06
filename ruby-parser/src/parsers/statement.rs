use crate::ast::{
    Alias, Conditional, ConditionalKind, Loop, LoopKind, Rescue, RescueClause, Undef,
};
use crate::lexer::*;
use crate::parsers::expression::assignment::assignment_statement;
use crate::parsers::expression::expression;
use crate::parsers::expression::method::defined_method_name;

/// *simple_statement* | *expression_modifier_statement* | *rescue_modifier_statement*
pub(crate) fn statement(i: Input) -> NodeResult {
    map(
        tuple((simple_statement, opt(recursing_statement_modifier))),
        Node::decurse,
    )(i)
}

/// *expression_statement* | *alias_statement* | *undef_statement* | *assignment_statement*
pub(crate) fn simple_statement(i: Input) -> NodeResult {
    alt((
        expression_statement,
        alias_statement,
        undef_statement,
        assignment_statement,
    ))(i)
}

pub(crate) fn recursing_statement_modifier(i: Input) -> NodeResult {
    map(
        tuple((
            alt((_expression_modifier_statement, _rescue_modifier_statement)),
            opt(recursing_statement_modifier),
        )),
        Node::decurse,
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
            ws0,
            method_name_or_symbol,
            ws0,
            method_name_or_symbol,
        )),
        |t| Node::Alias(Alias { to: t.2, from: t.4 }),
    )(i)
}

/// `undef` *undef_list*
pub(crate) fn undef_statement(i: Input) -> NodeResult {
    map(tuple((tag("undef"), ws0, undef_list)), |t| {
        Node::Undef(Undef { list: t.2 })
    })(i)
}

/// *method_name_or_symbol* ( [ no ⏎ ] `,` *method_name_or_symbol* )*
pub(crate) fn undef_list(i: Input) -> Parsed<Vec<String>> {
    map(
        tuple((
            method_name_or_symbol,
            many0(map(
                tuple((no_lt, char(','), ws0, method_name_or_symbol)),
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
pub(crate) fn method_name_or_symbol(i: Input) -> StringResult {
    preceded(opt(char(':')), defined_method_name)(i)
}

/// *statement* [ no ⏎ ] ( `if` | `unless` | `while` | `until` ) *expression*
pub(crate) fn _expression_modifier_statement(i: Input) -> NodeResult {
    map(
        tuple((
            no_lt,
            alt((tag("if"), tag("unless"), tag("while"), tag("until"))),
            ws0,
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
    map(tuple((no_lt, tag("rescue"), ws0, simple_statement)), |t| {
        Node::Rescue(Rescue {
            body: Box::new(Node::Placeholder),
            rescue: vec![RescueClause {
                exceptions: vec![],
                assigned_to: Box::new(Node::None),
                then: Box::new(t.3),
            }],
            otherwise: Box::new(Node::None),
        })
    })(i)
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
            Node::alias("foo?".to_owned(), "BAR".to_owned(),)
        );
        assert_ok!(
            "alias\n\nfoo\t:bar!",
            Node::alias("foo".to_owned(), "bar!".to_owned(),)
        );
        assert_ok!(
            "alias :sym func_name!",
            Node::alias("sym".to_owned(), "func_name!".to_owned(),)
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
        assert_ok!("undef foo?", Node::undef(vec!["foo?".to_owned()]));
        assert_ok!(
            "undef \n:bar   , BAZ\t,\n foo!",
            Node::undef(vec!["bar".to_owned(), "BAZ".to_owned(), "foo!".to_owned()])
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
        assert_ok!(
            "1 rescue 2",
            Node::rescued_statement(Node::int(1), Node::int(2))
        );
        assert_ok!(
            "1 rescue 2 rescue 3",
            Node::rescued_statement(
                Node::rescued_statement(Node::int(1), Node::int(2)),
                Node::int(3)
            )
        );
        assert_ok!(
            "1 rescue 2 or 3",
            Node::rescued_statement(Node::int(1), Node::logical_or(Node::int(2), Node::int(3)))
        );
        assert_ok!(
            "undef :hi if true rescue 3",
            Node::rescued_statement(
                Node::conditional(
                    ConditionalKind::ModifyingIf,
                    Node::boolean(true),
                    Node::undef(vec!["hi".to_owned()]),
                    Node::None
                ),
                Node::int(3)
            )
        );
        assert_ok!(
            "undef :hi rescue 3 if false",
            Node::conditional(
                ConditionalKind::ModifyingIf,
                Node::boolean(false),
                Node::rescued_statement(Node::undef(vec!["hi".to_owned()]), Node::int(3)),
                Node::None
            )
        );
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
