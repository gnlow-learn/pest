
use pest::{iterators::Pairs, pratt_parser::Assoc};
use pest::pratt_parser::PrattParser;
use pest::Parser;
use std::io::{self, BufRead};

#[derive(pest_derive::Parser)]
#[grammar = "pratt.pest"]
pub struct PRATTParser;

pub enum Op {
    Prefix(Rule, Assoc),
    Infix(Rule, Assoc),
    Postfix(Rule, Assoc),
}

fn main() {
    let pratt =
        PrattParser::new()
            .op(Op::Infix(Rule::add, Assoc::Left) | Op::Infix(Rule::sub, Assoc::Left))
            .op(Op::Infix(Rule::mul, Assoc::Left) | Op::Infix(Rule::div, Assoc::Left))
            .op(Op::Infix(Rule::pow, Assoc::Right))
            .op(Op::Postfix(Rule::fac))
            .op(Op::Prefix(Rule::neg));
    fn parse_expr(pairs: Pairs<Rule>, pratt: &PrattParser<Rule>) -> i32 {
        pratt
            .map_primary(|primary| match primary.as_rule() {
                Rule::int  => primary.as_str().parse().unwrap(),
                Rule::expr => parse_expr(primary.into_inner(), pratt), // from "(" ~ expr ~ ")"
                _          => unreachable!(),
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::neg  => -rhs,
                _          => unreachable!(),
            })
            .map_postfix(|lhs, op| match op.as_rule() {
                Rule::fac  => (1..lhs+1).product(),
                _          => unreachable!(),
            })
            .map_infix(|lhs, op, rhs| match op.as_rule() {
                Rule::add  => lhs + rhs,
                Rule::sub  => lhs - rhs,
                Rule::mul  => lhs * rhs,
                Rule::div  => lhs / rhs,
                Rule::pow  => (1..rhs+1).map(|_| lhs).product(),
                _          => unreachable!(),
            })
            .parse(pairs)
    }
    for line in io::stdin().lock().lines() {
        match PrattParser::parse(Rule::expr, &line?) {
            Ok(mut pairs) => {
                println!(
                    "Parsed: {:#?}",
                    parse_expr(
                        pairs.next().unwrap().into_inner(),
                        &PrattParser,
                    )
                );
            }
            Err(e) => {
                eprintln!("Parse failed: {:?}", e);
            }
        }
        Ok(())
    }
}