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
    alt((parse_integer, parse_op, parse_symbol, parse_list))(input)
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

fn eval_op(operation: LispOp, operands: Vec<u32>) -> Option<f32> {
    return match operation {
        LispOp::Div => {
            if operands.len() != 2 || operands[1] == 0 {
                None
            } else {
                Some((operands[0] / operands[1]) as f32)
            }
        }
        LispOp::Mul => {
            let mut acc: f32 = 1.0;

            for operand in operands {
                if operand == 0 {
                    return Some(0.0);
                } else {
                    acc *= operand as f32;
                }
            }
            Some(acc)
        }
        LispOp::Mod => {
            if operands.len() != 2 {
                None
            } else {
                Some((operands[0] % operands[1]) as f32)
            }
        }
        LispOp::Add => Some(operands.iter().sum::<u32>() as f32),
        LispOp::Sub => {
            if operands.len() != 2 {
                None
            } else {
                Some((operands[0] - operands[1]) as f32)
            }
        }
    };
}

fn evaluate(expression: LispExpr) -> i64 {
    match expression {
        // evaluate the function from here
        LispExpr::Symbol(_) => todo!(),
        LispExpr::Operation(_) => todo!(),
        LispExpr::Integer(value) => value,
        LispExpr::Float(_) => todo!(),
        LispExpr::List(list) => 43,
    }
}

fn main() {
    println!("Mahampreter v0.5");
    let mut rl = DefaultEditor::new().unwrap();
    let mut guess = String::new();

    loop {
        let prompt = format!("{}", Style::new().bold().paint("> "));
        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();

                let result = parse_expr(line.as_str());

                println!("{:?}", result);

                match result {
                    Ok((_, expr)) => {
                        println!("{}", evaluate(expr));
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
    use crate::{parse_expr, LispExpr::*, LispOp::*};

    #[test]
    fn double_numbers_list() {
        let result = "(+ 1 2)";

        assert_eq!(
            parse_expr(result),
            Ok(("", List([Operation(Add), Integer(1), Integer(2)].to_vec())))
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
}
