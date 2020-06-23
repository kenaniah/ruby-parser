/// Defines the functions used by the `assert_ok!`, `assert_partial!`, and `assert_err!` macros
#[macro_export]
macro_rules! use_parser {
    ($func:ident, $input_type:ty, $ok_type:ty, $err_type:ty) => {
        #[allow(dead_code)]
        fn parser(i: $input_type) -> nom::IResult<$input_type, $ok_type, ($input_type, $err_type)> {
            nom::combinator::all_consuming($func)(i)
        };
        #[allow(dead_code)]
        fn partial_parser(i: $input_type) -> nom::IResult<$input_type, $ok_type, ($input_type, $err_type)> {
            $func(i)
        };
        fn _type_check_ok(_expected: $ok_type) {}
        fn _type_check_err(_expected: $err_type) {}
    };
}

/// Tests whether the parser's output matches a successful result for the entire input
#[macro_export]
macro_rules! assert_ok {
    ($input:expr) => {
        let res = parser($input.into());
        if res.is_err() {
            panic!("\nExpected parsing to succeed...\n     input: {:?}\n    result: {:?}\n  expected: Ok", $input, res.unwrap_err())
        }
    };
    ($input:expr, $result:expr) => {
        _type_check_ok($result);
        let res = parser($input.into());
        if res.is_ok() {
            assert_eq!(parser($input.into()).unwrap().1, $result)
        } else {
            panic!("\nExpected parsing to succeed...\n     input: {:?}\n    result: {:?}\n  expected: {:?}", $input, res.unwrap_err(), $result)
        }
    };
}

/// Tests whether the parser's output matches a successful result for part of the input
#[macro_export]
macro_rules! assert_partial {
    ($input:expr) => {
        let res = partial_parser($input.into());
        if res.is_err() {
            panic!("\nExpected parsing to succeed...\n     input: {:?}\n    result: {:?}\n  expected: Ok", $input, res.unwrap_err())
        }
    };
    ($input:expr, $result:expr) => {
        _type_check_ok($result);
        let res = partial_parser($input.into());
        if res.is_ok() {
            assert_eq!(partial_parser($input.into()).unwrap().1, $result)
        } else {
            panic!("\nExpected parsing to succeed...\n     input: {:?}\n    result: {:?}\n  expected: {:?}", $input, res.unwrap_err(), $result)
        }
    };
}

/// Tests whether the parser's output matches an unsuccessful result
#[macro_export]
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
        _type_check_err($result);
        assert_eq!(
            parser($input.into()).unwrap_err(),
            nom::Err::Error(($remaining.into(), $result))
        )
    };
}
