use caltui_engine::ast::Expr::Literal;
use caltui_engine::ast::{
    BinaryOp::{Addition, Multiplication},
    Number, Value,
};

use caltui_engine::lexer::Token;

#[cfg(test)]
mod tests {

    use caltui_engine::{
        ast::{self, Expr},
        parser::Parser,
    };

    use super::*;

    #[test]
    fn to_reverse_polish_notation() {
        let parser = Parser::new();
        let input = vec![Token::Number(3), Token::Plus, Token::Number(4)];
        let result = parser
            .tokens_to_reverse_polish_notation(&input)
            .ok()
            .unwrap();
        let answer = vec![Token::Number(3), Token::Number(4), Token::Plus];
        assert_eq!(result, answer);

        let parser = Parser::new();
        let input = vec![
            Token::LeftParen,
            Token::Number(3),
            Token::Plus,
            Token::Number(4),
            Token::RightParen,
            Token::Star,
            Token::LeftParen,
            Token::Number(5),
            Token::Plus,
            Token::Number(6),
            Token::RightParen,
        ];
        let result = parser
            .tokens_to_reverse_polish_notation(&input)
            .ok()
            .unwrap();
        let answer = vec![
            Token::Number(3),
            Token::Number(4),
            Token::Plus,
            Token::Number(5),
            Token::Number(6),
            Token::Plus,
            Token::Star,
        ];
        assert_eq!(result, answer);
    }

    #[test]
    fn simple_ast() {
        let parser = Parser::new();
        let input = vec![
            Token::Number(3),
            Token::Number(4),
            Token::Plus,
            Token::Number(5),
            Token::Number(6),
            Token::Plus,
            Token::Star,
        ];
        let result = parser.rpn_to_ast(input);

        let correct = Expr::Binary {
            lhs: Box::new(Expr::Binary {
                lhs: Box::new(Literal(ast::Value::Number(Number { value: 3 }))),
                op: Addition,
                rhs: Box::new(Literal(Value::Number(Number { value: 4 }))),
            }),
            op: Multiplication,
            rhs: Box::new(Expr::Binary {
                lhs: Box::new(Literal(Value::Number(Number { value: 5 }))),
                op: Addition,
                rhs: Box::new(Literal(Value::Number(Number { value: 6 }))),
            }),
        };
        assert_eq!(result.ok(), Some(correct));
    }
}
