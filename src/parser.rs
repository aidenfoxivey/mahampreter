pub mod parser {
    use crate::{SchemeExpr, SchemeOp};

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, i64, multispace0, multispace1},
        combinator::{map, recognize},
        multi::separated_list0,
        number::complete::double,
        sequence::{delimited, pair},
        IResult,
    };

    fn parse_symbol(input: &str) -> IResult<&str, SchemeExpr> {
        map(recognize(alpha1), |s: &str| {
            SchemeExpr::Symbol(s.to_string())
        })(input)
    }

    fn parse_float(input: &str) -> IResult<&str, SchemeExpr> {
        map(double, SchemeExpr::Float)(input)
    }

    fn parse_integer(input: &str) -> IResult<&str, SchemeExpr> {
        map(i64, SchemeExpr::Integer)(input)
    }

    pub fn parse_expr(input: &str) -> IResult<&str, SchemeExpr> {
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
            recognize(alt((
                tag("*"),
                tag("/"),
                tag("+"),
                tag("-"),
                tag("%"),
                tag(">"),
                tag("<"),
                tag("<="),
                tag(">="),
                tag("="),
            ))),
            |s: &str| match s {
                "*" => SchemeExpr::Operation(SchemeOp::Mul),
                "/" => SchemeExpr::Operation(SchemeOp::Div),
                "+" => SchemeExpr::Operation(SchemeOp::Add),
                "-" => SchemeExpr::Operation(SchemeOp::Sub),
                "%" => SchemeExpr::Operation(SchemeOp::Mod),
                ">" => SchemeExpr::Operation(SchemeOp::Gt),
                "<" => SchemeExpr::Operation(SchemeOp::Lt),
                "<=" => SchemeExpr::Operation(SchemeOp::Leq),
                ">=" => SchemeExpr::Operation(SchemeOp::Geq),
                "=" => SchemeExpr::Operation(SchemeOp::Eq),
                _ => panic!("Invalid operator: {}", s),
            },
        )(input)
    }
}
