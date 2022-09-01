use std::collections::HashMap;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(grammar);

#[derive(Debug)]
pub enum CalcError {
    UnknownVariable(String),
    PowTooLarge,
}

fn main() {
    let mut state = HashMap::new();
    grammar::StmtParser::new()
        .parse(
            &mut state,
            "let a = 2201381908479023740917509813750817085337801471083271098371083712;",
        )
        .unwrap();
    let res = grammar::StmtParser::new()
        .parse(
            &mut state,
            "(a + 31490871094829048190438129038901839012839021809320 * 101329013810938920831) ^ 2;",
        )
        .unwrap();
    println!("{:?}", res);
}

#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test_parse() {}
}
