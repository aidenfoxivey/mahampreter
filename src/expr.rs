pub mod expr {
    use std::fmt;


    #[derive(Debug, PartialEq, Eq)]
    enum ValueErr {
        UnknownBase,
        OutOfBounds,
        UnknownFunction(String),
    }

    impl fmt::Display for ValueErr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ValueErr::UnknownBase => write!(f, "Only decimal, octal (#o), and binary (#b) are supported."),
                ValueErr::OutOfBounds => write!(f, "Number is too large to be supported."),
                ValueErr::UnknownFunction(string) => write!(f, "Unknown function: {}\n Have you forgotten something?", string),
            }
        }
    } 

    #[derive(Debug, PartialEq, Eq)]
    enum ArithErr {
        DivideByZero,
        BadOperator(char),
    }


    impl fmt::Display for ArithErr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ArithErr::DivideByZero => write!(f, "Illegal divide by zero operation."),
                ArithErr::BadOperator(ch) => write!(f, "{} is not a valid operator.", ch),
            }
        }
    } 

    #[derive(Debug, PartialEq, Clone)]
    pub enum SchemeOp {
        Div,
        Mul,
        Add,
        Sub,
        Mod,
        Gt,
        Lt,
        Leq,
        Geq,
        Eq,
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
            SchemeOp::Gt => todo!(),
            _ => todo!(),
        };
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum SchemeExpr {
        Symbol(String),
        Operation(SchemeOp),
        Integer(i64),
        Float(f64),
        List(Vec<SchemeExpr>),
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

    pub fn evaluate(expression: SchemeExpr) -> SchemeExpr {
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
}
