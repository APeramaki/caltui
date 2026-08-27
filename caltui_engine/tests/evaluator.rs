use caltui_engine::ast::Expr::Literal;
use caltui_engine::ast::{BinaryOp::Addition, Rational, Value};

#[cfg(test)]
mod tests {

    use std::assert_eq;

    use caltui_engine::{
        ast::{
            BinaryOp::{Division, Exponent, Multiplication, Subtraction},
            Expr,
            Value::{Frac, Integer},
        },
        evaluator::evaluate,
    };

    use super::*;

    #[test]
    fn evaluate_sum() {
        let input = Expr::Binary {
            lhs: Box::new(Expr::Binary {
                lhs: Box::new(Literal(Value::Integer(3))),
                op: Addition,
                rhs: Box::new(Literal(Value::Integer(4))),
            }),
            op: Addition,
            rhs: Box::new(Expr::Binary {
                lhs: Box::new(Literal(Value::Integer(5))),
                op: Addition,
                rhs: Box::new(Literal(Value::Integer(6))),
            }),
        };
        let output = evaluate(input).ok().unwrap();
        let answer = Value::Integer(18);
        assert_eq!(output, answer);
    }

    #[test]
    fn evaluate_add_of_fractionals() {
        let input: Expr = Expr::Binary {
            lhs: Box::new(Literal(Frac(Rational::new(5, 2)))),
            op: Addition,
            rhs: Box::new(Literal(Frac(Rational::new(10, 4)))),
        };
        let output = evaluate(input);
        // 5/2 + 10/4 reduces to a whole number, so it collapses to an Integer.
        let answer = Value::Integer(5);

        assert_eq!(output, Ok(answer));
    }

    #[test]
    fn evaluate_mul_of_fractionals() {
        let input: Expr = Expr::Binary {
            lhs: Box::new(Literal(Frac(Rational::new(5, 2)))),
            op: Multiplication,
            rhs: Box::new(Literal(Frac(Rational::new(11, 4)))),
        };
        let output = evaluate(input);
        let answer = Value::Frac(Rational::new(55, 8));

        assert_eq!(output, Ok(answer));
    }

    #[test]
    fn evaluate_div_of_fractionals() {
        let input: Expr = Expr::Binary {
            lhs: Box::new(Literal(Frac(Rational::new(5, 2)))),
            op: Division,
            rhs: Box::new(Literal(Frac(Rational::new(11, 4)))),
        };
        let output = evaluate(input);
        let answer = Value::Frac(Rational::new(10, 11));

        assert_eq!(output, Ok(answer));
    }

    #[test]
    fn evaluate_sub_of_fractionals() {
        let input: Expr = Expr::Binary {
            lhs: Box::new(Literal(Frac(Rational::new(5, 2)))),
            op: Subtraction,
            rhs: Box::new(Literal(Frac(Rational::new(11, 4)))),
        };
        let output = evaluate(input);
        let answer = Value::Frac(Rational::new(-1, 4));

        assert_eq!(output, Ok(answer));
    }

    #[test]
    fn evaluate_add_of_integer_and_fractional() {
        let input: Expr = Expr::Binary {
            lhs: Box::new(Literal(Integer(2))),
            op: Addition,
            rhs: Box::new(Literal(Frac(Rational::new(11, 4)))),
        };
        let output = evaluate(input);
        let answer = Value::Frac(Rational::new(19, 4));

        assert_eq!(output, Ok(answer));
    }

    #[test]
    fn evaluate_add_of_fractional_and_integer() {
        let input: Expr = Expr::Binary {
            lhs: Box::new(Literal(Frac(Rational::new(11, 4)))),
            op: Addition,
            rhs: Box::new(Literal(Integer(2))),
        };
        let output = evaluate(input);
        let answer = Value::Frac(Rational::new(19, 4));

        assert_eq!(output, Ok(answer));
    }

    #[test]
    fn evaluate_integer_pow() {
        let input: Expr = Expr::Binary {
            lhs: Box::new(Literal(Integer(2))),
            op: Exponent,
            rhs: Box::new(Literal(Integer(3))),
        };
        let answer = Value::Integer(8);
        assert_eq!(evaluate(input), Ok(answer));
    }

    #[test]
    fn evaluate_integer_third_root() {
        let input = Expr::Binary {
            lhs: Box::new(Literal(Integer(8))),
            op: Exponent,
            rhs: Box::new(Expr::Literal(Value::Frac(Rational::new(1, 3)))),
        };
        let answer = Value::Integer(2);
        assert_eq!(evaluate(input), Ok(answer));
    }

    #[test]
    fn evaluate_fractional_base_to_fractional_power() {
        let input = Expr::Binary {
            lhs: Box::new(Literal(Frac(Rational::new(8, 27)))),
            op: Exponent,
            rhs: Box::new(Literal(Frac(Rational::new(2, 3)))),
        };
        let answer = Value::Frac(Rational::new(4, 9));
        assert_eq!(evaluate(input), Ok(answer));
    }
}
