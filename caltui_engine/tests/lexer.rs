use caltui_engine::{ast::BinaryOp, lexer::Operator, lexer::lexer};
#[cfg(test)]
mod tests {
    use caltui_engine::lexer::Token;

    use super::*;

    #[test]
    fn number() {
        let result = lexer("5");

        assert_eq!(result, Ok(vec![Token::Number(5)]));
    }
    #[test]
    fn operator() {
        let result = lexer("+");

        assert_eq!(result, Ok(vec![Token::Plus]));

        assert_eq!(
            result.unwrap().first().unwrap().as_operator(),
            Some(Operator::Binary(BinaryOp::Addition))
        );
    }

    #[test]
    fn sum() {
        let result = lexer("1 + 2");

        assert_eq!(
            result,
            Ok(vec![Token::Number(1), Token::Plus, Token::Number(2)])
        );
    }

    #[test]
    fn identifier() {
        let result = lexer("1 * g");

        assert_eq!(
            result,
            Ok(vec![
                Token::Number(1),
                Token::Star,
                Token::Identifier("g".to_string())
            ])
        );
    }

    #[test]
    fn multidigit() {
        let result = lexer("1 * 123");

        assert_eq!(
            result,
            Ok(vec![Token::Number(1), Token::Star, Token::Number(123)])
        );
    }
    #[test]
    fn multicharacter() {
        let result = lexer("1 * abc + 123");

        assert_eq!(
            result,
            Ok(vec![
                Token::Number(1),
                Token::Star,
                Token::Identifier(String::from("abc")),
                Token::Plus,
                Token::Number(123)
            ])
        );
    }
}
