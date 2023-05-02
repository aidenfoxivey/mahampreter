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

fn eval_op(operation: LispOp, operands: Vec<i64>) -> Option<i64> {
    return match operation {
        LispOp::Div => {
            if operands.len() != 2 || operands[1] == 0 {
                None
            } else {
                Some((operands[0] / operands[1]) as i64)
            }
        }
        LispOp::Mul => {
            let mut acc = 1;

            for operand in operands {
                if operand == 0 {
                    return Some(0);
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
        LispOp::Add => Some(operands.iter().sum::<i64>()),
        LispOp::Sub => {
            if operands.len() != 2 {
                None
            } else {
                Some(operands[0] - operands[1])
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
        LispExpr::List(list) => {
            let iter = list.iter();
            let first_expr = list[0].clone();

            let iter = iter.skip(1); // skip the operand
            let mut operands: Vec<i64> = vec![];

            for expr in iter {
                operands.push(evaluate(expr.clone()));
            }

            let op;

            match first_expr {
                LispExpr::Operation(op_type) => {op = op_type},
                _ => todo!(),
            } 

            eval_op(op, operands).unwrap()

        },
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
