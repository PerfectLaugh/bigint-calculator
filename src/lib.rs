#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    pub(crate) grammar
);

#[derive(Debug)]
pub enum CalcError {
    UnknownVariable(String),
    PowTooLarge,
}

pub use grammar::StmtParser as Parser;

#[cfg(test)]
mod test {
    use super::*;
    use num_bigint::BigInt;
    use std::collections::HashMap;

    #[test]
    fn test_pass_parse_1() {
        let parser = Parser::new();
        let mut state = HashMap::new();
        // u64::MAX + 1
        parser
            .parse(&mut state, "let a = 18446744073709551616;")
            .expect("parse error");
        let res = parser
            .parse(
                &mut state,
                "a ^ 3 + 18446744073709551616 ^ 10 * (18446744073709551616 ^ 12 + 10);",
            )
            .expect("parse error");
        assert_eq!(res.expect("no result"), BigInt::parse_bytes(b"100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000", 16).unwrap());
    }

    #[test]
    fn test_pass_parse_2() {
        let parser = Parser::new();
        let mut state = HashMap::new();
        parser
            .parse(&mut state, "let b = 18446744073709551616;")
            .expect("parse error");

        parser
            .parse(
                &mut state,
                "let b = b ^ 0b11 + 18446744073709551616 ^ 10 * (18446744073709551616 ^ 12 + 10);",
            )
            .expect("parse error");
        let res = parser.parse(&mut state, "b;").expect("parse error");
        assert_eq!(res.expect("no result"), BigInt::parse_bytes(b"100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000", 16).unwrap());
    }

    #[test]
    fn test_fail_pow_too_large() {
        let parser = Parser::new();
        let mut state = HashMap::new();
        assert!(parser
            .parse(
                &mut state,
                "2201381908479023740917509813750817085337801471083271098371083712 ^ 142901481904819048291841;",
            ).is_err())
    }
}
