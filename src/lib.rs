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
        parser
            .parse(
                &mut state,
                "let a = 2201381908479023740917509813750817085337801471083271098371083712;",
            )
            .expect("parse error");
        let res = parser
            .parse(
                &mut state,
                "a ^ 3 + 31490871094829048190438129038901839012839021809320 ^ 10 * (101329013810938920831 ^ 12);",
            )
            .expect("parse error");
        assert_eq!(res.expect("no result"), BigInt::parse_bytes(b"37279f2600e483089fb551b51383c386ab45193612566b0a4b4dc074c05988e45c80bff0493e85e108430ff4db9a4fd2984f49b499cbeeec30bb7598a9f4f1d47f2f5a68ce56a848a4777e7fdd06daff85749c787bb3ebff5de4ee10dfbd165ca7f94a738965719a7e8119f4a3176cd27e6d0751d5f5fae2bc7a20405fad0c1170fefe2e314f94b5285adc12fe8185e5c793733e069e033722cc1e20445125616b590df92fcd73ae620814aa3e4a62ed9b44ecd769923783c5ff2803eba48ce2f08044efb3be82a1fa89e297b06028f2136a70b4c08254368bcb3def9171bf07295918ddfe94ac37666b2de6143bf8a37650a8343331851913b25bada3348a11f6787754137f6f04eb4392ab38a0997065ff2ab851bf8f5d3e0db8e6b26404363c216feac8a5aabe72ac7a48918f79c0000", 16).unwrap());
    }

    #[test]
    fn test_pass_parse_2() {
        let parser = Parser::new();
        let mut state = HashMap::new();
        parser
            .parse(
                &mut state,
                "let b = 2201381908479023740917509813750817085337801471083271098371083712;",
            )
            .expect("parse error");

        parser.parse(
                &mut state,
                "let b = b ^ 0b11 + 0x158c037f6e1c6c2e2742d8de291705bee517970ea8 ^ 10 * (0o12770707623115152437577 ^ 12);",
            )
            .expect("parse error");
        let res = parser.parse(&mut state, "b;").expect("parse error");
        assert_eq!(res.expect("no result"), BigInt::parse_bytes(b"37279f2600e483089fb551b51383c386ab45193612566b0a4b4dc074c05988e45c80bff0493e85e108430ff4db9a4fd2984f49b499cbeeec30bb7598a9f4f1d47f2f5a68ce56a848a4777e7fdd06daff85749c787bb3ebff5de4ee10dfbd165ca7f94a738965719a7e8119f4a3176cd27e6d0751d5f5fae2bc7a20405fad0c1170fefe2e314f94b5285adc12fe8185e5c793733e069e033722cc1e20445125616b590df92fcd73ae620814aa3e4a62ed9b44ecd769923783c5ff2803eba48ce2f08044efb3be82a1fa89e297b06028f2136a70b4c08254368bcb3def9171bf07295918ddfe94ac37666b2de6143bf8a37650a8343331851913b25bada3348a11f6787754137f6f04eb4392ab38a0997065ff2ab851bf8f5d3e0db8e6b26404363c216feac8a5aabe72ac7a48918f79c0000", 16).unwrap());
    }
}
