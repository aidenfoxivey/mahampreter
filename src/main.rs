#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]

// TODO: implement keywords

use ansi_term::Style;
use clap::Parser;
use nom::character::complete::i64;
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
use std::fmt;
use std::{process::exit, vec};

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Eq, PartialEq)]
enum MathError {
    DivideByZero,
    TooLarge,
}

enum SchemeError {}

#[derive(Debug, Clone)]
struct DoubleError;

#[derive(Debug, PartialEq, Clone)]
enum SchemeOp {
    Div,
    Mul,
    Add,
    Sub,
    Mod,
}

#[derive(Debug, PartialEq, Clone)]
enum SchemeExpr {
    Symbol(String),
    Operation(SchemeOp),
    Integer(i64),
    Float(f64),
    List(Vec<SchemeExpr>),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scheme source file
    file: Option<String>,

    /// disable all warnings
    #[arg(short = 'w', long = "no-warnings", default_value_t = false)]
    no_warnings: bool,

    /// do not print the banner
    #[arg(short = 'q', long = "quiet", default_value_t = false)]
    no_banner: bool,

    /// pretty print Scheme expression
    #[arg(short = 'p', long = "pretty-print", value_name = "EXPRESSION")]
    pretty_expr: Option<String>,

    /// evaluate Scheme expression
    #[arg(short = 'e', long = "evaluate", value_name = "EXPRESSION")]
    expr: Option<String>,
}

impl fmt::Display for SchemeExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemeExpr::Symbol(symbol) => write!(f, "{}", symbol),
            SchemeExpr::Operation(operation) => todo!(),
            SchemeExpr::Integer(value) => write!(f, "{}", value),
            SchemeExpr::Float(value) => write!(f, "{}", value),
            SchemeExpr::List(list) => {
                // let mut contents = String::new();

                // for obj in list {
                //     contents.push_str(format!("{} ", obj))
                // }
                // collect as an iterator?
                // write!(f, "()")
                todo!()
            }
        }
    }
}

fn parse_symbol(input: &str) -> IResult<&str, SchemeExpr> {
    map(recognize(alpha1), |s: &str| {
        SchemeExpr::Symbol(s.to_string())
    })(input)
}

fn parse_float(input: &str) -> IResult<&str, SchemeExpr> {
    map(double, SchemeExpr::Float)(input)
}

// TODO: add support for negative integers too
fn parse_integer(input: &str) -> IResult<&str, SchemeExpr> {
    map(i64, SchemeExpr::Integer)(input)
}

fn parse_expr(input: &str) -> IResult<&str, SchemeExpr> {
    alt((
        parse_float,
        parse_integer,
        parse_op,
        parse_symbol,
        parse_list,
    ))(input)
}

fn parse_list(input: &str) -> IResult<&str, SchemeExpr> {
    map(
        delimited(
            pair(tag("("), multispace0),
            separated_list0(multispace1, parse_expr),
            pair(multispace0, tag(")")),
        ),
        SchemeExpr::List,
    )(input)
}

fn parse_op(input: &str) -> IResult<&str, SchemeExpr> {
    map(
        recognize(alt((tag("*"), tag("/"), tag("+"), tag("-"), tag("%")))),
        |s: &str| match s {
            "*" => SchemeExpr::Operation(SchemeOp::Mul),
            "/" => SchemeExpr::Operation(SchemeOp::Div),
            "+" => SchemeExpr::Operation(SchemeOp::Add),
            "-" => SchemeExpr::Operation(SchemeOp::Sub),
            "%" => SchemeExpr::Operation(SchemeOp::Mod),
            _ => panic!("Incorrect operator parsing."),
        },
    )(input)
}

fn eval_op(operation: SchemeOp, expressions: Vec<SchemeExpr>) -> Option<f64> {
    let mut operands = vec![];

    for expr in expressions {
        match expr {
            SchemeExpr::Symbol(_) => todo!(),
            SchemeExpr::Operation(_) => todo!(),
            SchemeExpr::Integer(value) => operands.push(value as f64),
            SchemeExpr::Float(value) => operands.push(value),
            SchemeExpr::List(_) => todo!(),
        }
    }

    return match operation {
        SchemeOp::Div => {
            if operands.len() != 2 || operands[1] == 0.0 {
                None
            } else {
                Some(((operands[0] / operands[1]) as i64) as f64)
            }
        }
        SchemeOp::Mul => {
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
        SchemeOp::Mod => {
            if operands.len() != 2 {
                None
            } else {
                Some(operands[0] % operands[1])
            }
        }
        SchemeOp::Add => Some(operands.iter().sum::<f64>()),
        SchemeOp::Sub => {
            if operands.len() != 2 {
                None
            } else {
                Some(operands[0] - operands[1])
            }
        }
    };
}

fn evaluate(expression: SchemeExpr) -> SchemeExpr {
    match expression {
        SchemeExpr::Symbol(_) => todo!(),
        SchemeExpr::Operation(_) => expression,
        SchemeExpr::Integer(_) => expression,
        SchemeExpr::Float(_) => expression,
        SchemeExpr::List(list) => {
            if list.is_empty() {
                return SchemeExpr::Symbol("NIL".to_string());
            }

            let iter = list.iter();
            let first_expr = list[0].clone();

            let iter = iter.skip(1); // skip the operand
            let mut operands: Vec<SchemeExpr> = vec![];

            for expr in iter {
                match evaluate(expr.clone()) {
                    SchemeExpr::Symbol(_) => todo!(),
                    SchemeExpr::Operation(_) => todo!(),
                    SchemeExpr::Integer(value) => operands.push(SchemeExpr::Integer(value)),
                    SchemeExpr::Float(value) => operands.push(SchemeExpr::Float(value)),
                    SchemeExpr::List(_) => todo!(),
                }
            }

            let op: SchemeOp = match first_expr {
                SchemeExpr::Operation(op_type) => op_type,
                _ => todo!(),
            };

            SchemeExpr::Float(eval_op(op, operands).unwrap())
        }
    }
}

fn main() {
    let args = Args::parse();

    println!("mahampreter v0.0.5");
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
                        println!("{}", evaluate(expr));
                    }
                    Err(error) => println!("{:?}", error),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl+C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl+D");
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
    use crate::{evaluate, parse_expr, SchemeExpr, SchemeExpr::*, SchemeOp::*};

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

        assert_eq!(parse_expr(result), Ok(("", List([].to_vec()))));
    }

    #[test]
    fn nested_execution() {
        let input = "(+ 1 2 (+ 1 1 1))";
        let (_, parsed) = parse_expr(input).unwrap();

        assert_eq!(evaluate(parsed), SchemeExpr::Float(6.0));
    }

    #[test]
    fn floating_point_addition() {
        let input = "(+ 1.0 2.0)";
        let (_, parsed) = parse_expr(input).unwrap();
        assert_eq!(evaluate(parsed), SchemeExpr::Float(3.0));
    }

    #[test]
    fn nil_case() {
        let input = "()";
        let (_, parsed) = parse_expr(input).unwrap();
        assert_eq!(evaluate(parsed), SchemeExpr::Symbol("NIL".to_string()));
    }
}
