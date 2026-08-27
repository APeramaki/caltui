pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {

    use crate::ast::{Rational, Value};

    use super::*;

    #[test]
    fn minimal() {
        let mut calc = evaluator::Calculator::new();
        let result = calc.execute("5");
        let parse_error = calc.execute("+");
        assert_eq!(result, Ok(ast::Value::Integer(5)));
        assert_eq!(
            parse_error,
            Err(evaluator::CalcError::Parse(
                parser::ParseError::MissingOperand
            ))
        );
    }

    #[test]
    fn parenthesis() {
        let mut calc = evaluator::Calculator::new();
        let result = calc.execute("(4*5)/2");
        let answer = Ok(Value::Integer(10));
        assert_eq!(result, answer);
    }
    #[test]
    fn mixed_calculation() {
        let mut calc = evaluator::Calculator::new();
        let result = calc.execute("(8/27)^(2/3)");

        assert_eq!(result, Ok(ast::Value::Frac(Rational::new(4, 9))));
    }
}
