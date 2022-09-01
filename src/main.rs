#[macro_use]
extern crate pest_derive;

use num_bigint::BigInt;

use pest::Parser;
use pest::iterators::Pair;

enum AST {
    Ident(String),
    Number(BigInt),
    Add(Box<AST>, Box<AST>),
    Sub(Box<AST>, Box<AST>),
    Mul(Box<AST>, Box<AST>),
    Div(Box<AST>, Box<AST>),
    Mod(Box<AST>, Box<AST>),
    Pow(Box<AST>, Box<AST>),
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ProgParser;

pub struct Calculator {
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator {}
    }

    pub fn calculate(&self, input: &str) -> Result<BigInt, String> {
        let pairs = ProgParser::parse(Rule::prog, input)
            .map_err(|e| format!("Error parsing input: {}", e))?;

        let mut result = BigInt::from(0);
        for pair in pairs {
            match pair.as_rule() {
                Rule::number => {
                    result = pair.as_str().parse::<BigInt>().unwrap();
                }
                Rule::add => {
                    result += pair.as_str().parse::<BigInt>().unwrap();
                }
                Rule::sub => {
                    result -= pair.as_str().parse::<BigInt>().unwrap();
                }
                Rule::mul => {
                    result *= pair.as_str().parse::<BigInt>().unwrap();
                }
                Rule::div => {
                    result /= pair.as_str().parse::<BigInt>().unwrap();
                }
                _ => unreachable!(),
            }
        }

        Ok(result)
    }

    fn visit_term<'s>(&self, pair: Pair<'s, Rule>, mut operand: BigInt) -> BigInt {
        let rule = pair.as_rule();
        assert_eq!(rule, Rule::term);
        let inner = pair.into_inner();

        for pair in inner {
            match pair.as_rule() {
                Rule::term => {
                    operand = self.visit_term(pair, operand);
                }
                Rule::add => {

                }
                _ => unimplemented!()
            }
        }
        match inner {
            Rule::term => {
                self.visit_term(, num)
            }
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let res = ProgParser::parse(Rule::prog, "(t * 20) + 30");
    println!("{:?}", res);
}

#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test_parse() {
    }
}
