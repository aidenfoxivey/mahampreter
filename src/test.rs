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
