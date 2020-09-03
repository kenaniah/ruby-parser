#[macro_export]
macro_rules! stack_frame {
    ($name:expr, $input:expr) => {{
        // let mut i = $input.clone();
        // let padding = std::iter::repeat("  ")
        //     .take(i.metadata.stack_depth)
        //     .collect::<String>();
        // i.metadata.stack_depth = i.metadata.stack_depth + 1;
        // println!("{}in {}: {}", padding, $name, $input);
        // i
        $input
    }};
}

/// Allows placeholding nodes to be updated when working around left-recursive parsers
#[macro_export]
macro_rules! update_placeholder {
    ($value:expr, $node:expr) => {
        if let Some(mut parent_node) = $node {
            use std::borrow::BorrowMut;
            {
                let mut n = &mut parent_node;
                loop {
                    match n {
                        Node::Conditional(sub) => n = sub.cond.borrow_mut(),
                        Node::BinaryOp(sub) => n = sub.lhs.borrow_mut(),
                        Node::LogicalOr(sub) => n = sub.first.borrow_mut(),
                        Node::LogicalAnd(sub) => n = sub.first.borrow_mut(),
                        Node::LogicalNot(sub) => n = sub.expr.borrow_mut(),
                        _ => break,
                    }
                }
                *n = $value;
            }
            parent_node
        } else {
            $value
        }
    };
}

/// Defines the functions used by the `assert_ok!`, `assert_partial!`, and `assert_err!` macros
#[macro_export]
macro_rules! use_parser {
    ($func:ident) => {
        #[allow(dead_code)]
        fn ident<T>(i: T) -> T {
            i
        }
        use_parser!($func, ident);
    };
    ($func:ident, $output:ident) => {
        use_parser!($func, $output, crate::Input, crate::ErrorKind);
    };
    ($func:ident, $output:ident, $input_type:ty, $err_type:ty) => {

        #[allow(unused_variables)]
        let mut parser = nom::combinator::all_consuming($func);
        #[allow(unused_variables)]
        let partial_parser = $func;

        /// Tests whether the parser's output matches a successful result for the entire input
        #[allow(unused_macros)]
        macro_rules! assert_ok {
            ($input:expr) => {
                let res = parser($input.into());
                if res.is_err() {
                    panic!("\nExpected parsing to succeed...\n     input: {:?}\n    result: {:?}\n  expected: Ok", $input, res.unwrap_err())
                }
            };
            ($input:expr, $result:expr) => {
                let res = parser($input.into());
                if res.is_ok() {
                    assert_eq!($output(res.unwrap().1), $result)
                } else {
                    panic!("\nExpected parsing to succeed...\n     input: {:?}\n    result: {:?}\n  expected: {:?}", $input, res.unwrap_err(), $result)
                }
            };
        };

        /// Tests whether the parser's output matches a successful result for part of the input
        #[allow(unused_macros)]
        macro_rules! assert_partial {
            ($input:expr) => {
                let res = partial_parser($input.into());
                if res.is_err() {
                    panic!("\nExpected parsing to succeed...\n     input: {:?}\n    result: {:?}\n  expected: Ok", $input, res.unwrap_err())
                }
            };
            ($input:expr, $result:expr) => {
                let res = partial_parser($input.into());
                if res.is_ok() {
                    assert_eq!($output(res.unwrap().1), $result)
                } else {
                    panic!("\nExpected parsing to succeed...\n     input: {:?}\n    result: {:?}\n  expected: {:?}", $input, res.unwrap_err(), $result)
                }
            };
        };

        /// Tests that the parser consumes the expected amount of input
        #[allow(unused_macros)]
        macro_rules! assert_remaining {
            ($input:expr, $remaining:expr) => {
                let res = partial_parser($input.into());
                if res.is_ok() {
                    assert_eq!(*$output(res.unwrap().0), $remaining)
                } else {
                    panic!("\nExpected parsing to succeed...\n     input: {:?}\n    result: {:?}\n  expected: {:?}", $input, res.unwrap_err(), $remaining)
                }
            }
        }

        /// Tests whether the parser's output matches an unsuccessful result
        #[allow(unused_macros)]
        macro_rules! assert_err {
            ($input:expr) => {
                let res = parser($input.into());
                if res.is_ok() {
                    panic!(
                        "\nExpected parsing to fail...\n     input: {:?}\n    result: {:?}\n  expected: Err",
                        $input,
                        res.unwrap().1
                    );
                }
            };
            ($input:expr, $remaining:expr, $result:expr) => {
                assert_eq!(
                    parser($input.into()).unwrap_err(),
                    nom::Err::Error(($remaining.into(), $result))
                )
            };
        };

        #[allow(unused_macros)]
        macro_rules! parser {
            ($input:expr) => { parser($input.into()) }
        }

    };
}
