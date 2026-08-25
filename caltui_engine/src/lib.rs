pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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
}
