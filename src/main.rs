#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]

// TODO: implement keywords

use std::{process::exit, vec};
use ansi_term::Style;
use nom::{
    branch::alt,
    character::complete::{alpha1, digit1, multispace0, multispace1, space0},
    combinator::{map, recognize},
    multi::{many0, separated_list0},
    number::complete::{double, float},
    sequence::{delimited, pair, tuple},
    {bytes::complete::tag, IResult},
};
use rustyline::{error::ReadlineError, DefaultEditor};

#[derive(Debug, PartialEq, Clone)]
enum LispOp {
    Div,
    Mul,
    Add,
    Sub,
    Mod,
}

#[derive(Debug, PartialEq, Clone)]
enum LispExpr {
    Symbol(String),
    Operation(LispOp),
    Integer(i64),
    Float(f64),
    List(Vec<LispExpr>),
}

fn parse_symbol(input: &str) -> IResult<&str, LispExpr> {
    map(recognize(alpha1), |s: &str| LispExpr::Symbol(s.to_string()))(input)
}

fn parse_float(input: &str) -> IResult<&str, LispExpr> {
    map(double, LispExpr::Float)(input)
}

// TODO: add support for negative integers too
fn parse_integer(input: &str) -> IResult<&str, LispExpr> {
    map(digit1, |s: &str| {
        LispExpr::Integer(s.trim().parse::<i64>().unwrap())
    })(input)
}

fn parse_expr(input: &str) -> IResult<&str, LispExpr> {
    alt((parse_float, parse_integer, parse_op, parse_symbol, parse_list))(input)
}

fn parse_list(input: &str) -> IResult<&str, LispExpr> {
    map(
        delimited(
            pair(tag("("), multispace0),
            separated_list0(multispace1, parse_expr),
            pair(multispace0, tag(")")),
        ),
        LispExpr::List,
    )(input)
}

fn parse_op(input: &str) -> IResult<&str, LispExpr> {
    map(
        recognize(alt((tag("*"), tag("/"), tag("+"), tag("-"), tag("%")))),
        |s: &str| match s {
            "*" => LispExpr::Operation(LispOp::Mul),
            "/" => LispExpr::Operation(LispOp::Div),
            "+" => LispExpr::Operation(LispOp::Add),
            "-" => LispExpr::Operation(LispOp::Sub),
            "%" => LispExpr::Operation(LispOp::Mod),
            _ => panic!("Incorrect operator parsing."),
        },
    )(input)
}

fn eval_op(operation: LispOp, expressions: Vec<LispExpr>) -> Option<f64> {

    let mut operands = vec![];

    for expr in expressions {
        match expr {
            LispExpr::Symbol(_) => todo!(),
            LispExpr::Operation(_) => todo!(),
            LispExpr::Integer(value) => operands.push(value as f64),
            LispExpr::Float(value) => operands.push(value),
            LispExpr::List(_) => todo!(),
        }
    }

    return match operation {
        LispOp::Div => {
            if operands.len() != 2 || operands[1] == 0.0 {
                None
            } else {
                Some(((operands[0] / operands[1]) as i64) as f64)
            }
        }
        LispOp::Mul => {
            let mut acc = 1.0;

            for operand in operands {
                if operand == 0.0 {
                    return Some(0.0);
                } else {
                    acc *= operand;
                }
            }
            Some(acc)
        }
        LispOp::Mod => {
            if operands.len() != 2 {
                None
            } else {
                Some(operands[0] % operands[1])
            }
        }
        LispOp::Add => Some(operands.iter().sum::<f64>()),
        LispOp::Sub => {
            if operands.len() != 2 {
                None
            } else {
                Some(operands[0] - operands[1])
            }
        }
    };
}

fn evaluate(expression: LispExpr) -> LispExpr {
    match expression {
        LispExpr::Symbol(_) => todo!(),
        LispExpr::Operation(_) => expression,
        LispExpr::Integer(_) => expression,
        LispExpr::Float(_) => expression,
        LispExpr::List(list) => {

            if list.len() == 0 {
                return LispExpr::Symbol("NIL".to_string());
            }

            let iter = list.iter();
            let first_expr = list[0].clone();

            let iter = iter.skip(1); // skip the operand
            let mut operands: Vec<LispExpr> = vec![];

            for expr in iter {
                match evaluate(expr.clone()) {
                    LispExpr::Symbol(_) => todo!(),
                    LispExpr::Operation(_) => todo!(),
                    LispExpr::Integer(value) => operands.push(LispExpr::Integer(value)),
                    LispExpr::Float(value) => operands.push(LispExpr::Float(value)),
                    LispExpr::List(_) => todo!(),
                }
            }

            let op: LispOp;

            match first_expr {
                LispExpr::Operation(op_type) => {op = op_type},
                _ => todo!(),
            } 

            LispExpr::Float(eval_op(op, operands).unwrap())

        },
    }
}

fn main() {
    println!("Mahampreter v0.5");
    println!("Press Ctrl-C to exit the REPL.\n");
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let prompt = format!("😺 {}", Style::new().bold().paint("> "));
        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();

                let result = parse_expr(line.as_str());

                match result {
                    Ok((_, expr)) => {
                        println!("{:?}", evaluate(expr));
                    }
                    Err(error) => println!("{:?}", error),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_expr, evaluate, LispExpr, LispExpr::*, LispOp::*};

    #[test]
    fn double_numbers_list() {
        let result = "(+ 1 2)";

        assert_eq!(
            parse_expr(result),
            Ok(("", List([Operation(Add), Float(1.0), Float(2.0)].to_vec())))
        );
    }

    #[test]
    fn empty_list() {
        let result = "()";

        assert_eq!(
            parse_expr(result),
            Ok(("", List([].to_vec())))
        );
    }

    #[test]
    fn nested_execution() {
        let input = "(+ 1 2 (+ 1 1 1))"; 
        let (_, parsed) = parse_expr(input).unwrap();

        assert_eq!(evaluate(parsed), LispExpr::Float(6.0));
    }

    #[test]
    fn floating_point_addition() {
        let input = "(+ 1.0 2.0)";
        let (_, parsed) = parse_expr(input).unwrap();
        assert_eq!(evaluate(parsed), LispExpr::Float(3.0));
    } 

    #[test]
    fn nil_case() {
        let input = "()";
        let (_, parsed) = parse_expr(input).unwrap();
        assert_eq!(evaluate(parsed), LispExpr::Symbol("NIL".to_string()));
    }
}
