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
        let parse_error = calc.execute("lol");
        assert_eq!(result, Ok(ast::Number { value: 5 }));
        assert_eq!(parse_error, Err(evaluator::CalcError::ParseIntError))
    }
}
