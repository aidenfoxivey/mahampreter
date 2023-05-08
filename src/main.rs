pub mod expr;
pub mod parser;
pub mod test;

use crate::expr::expr::evaluate;
use ansi_term::Style;
use clap::Parser;
use rustyline::{error::ReadlineError, DefaultEditor};


use crate::expr::expr::{SchemeExpr, SchemeOp};
use crate::parser::parser::parse_expr;

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

// #[derive(Debug, Eq, PartialEq)]
// enum MathError {
//     DivideByZero,
//     TooLarge,
// }

// enum SchemeError {}

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
