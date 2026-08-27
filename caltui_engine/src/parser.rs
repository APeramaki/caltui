use std::{cmp::Ordering, todo};

use crate::{
    ast::{self, Expr},
    lexer::{
        Associativity, Operator,
        Token::{self, Identifier},
    },
};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingParenthesis,
    MissingOperand,
    InvalidExpression,
}

pub struct Parser {}
impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
impl Parser {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
    // Build AST with Shunting yard algorithm
    pub fn build_ast(&self, tokens: &[Token]) -> Result<Expr, ParseError> {
        let rpn = self.tokens_to_reverse_polish_notation(tokens)?;

        self.rpn_to_ast(rpn)
    }

    pub fn rpn_to_ast(&self, rpn: Vec<Token>) -> Result<Expr, ParseError> {
        let mut stack: Vec<Expr> = Vec::new();

        for token in rpn {
            match token.as_operator() {
                Some(Operator::Binary(op)) => {
                    let rhs = stack.pop().ok_or(ParseError::MissingOperand)?;
                    let lhs = stack.pop().ok_or(ParseError::MissingOperand)?;

                    stack.push(Expr::Binary {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                    });
                }

                Some(Operator::Unary(_op)) => todo!(),

                None => match token {
                    Token::Number(n) => {
                        stack.push(Expr::Literal(ast::Value::Integer(n)));
                    }
                    _ => todo!(),
                },
            }
        }

        match stack.pop() {
            Some(expr) if stack.is_empty() => Ok(expr),
            _ => Err(ParseError::InvalidExpression),
        }
    }
    pub fn tokens_to_reverse_polish_notation(
        &self,
        tokens: &[Token],
    ) -> Result<Vec<Token>, ParseError> {
        let mut output: Vec<Token> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();
        let mut tokens = tokens.iter().peekable();

        while let Some(token) = tokens.next() {
            match token {
                Token::Number(n) => output.push(Token::Number(*n)),

                Token::Identifier(ident)
                    if tokens.peek().is_some_and(|t| **t == Token::LeftParen) =>
                {
                    output.push(Token::Function(ident.to_string()));
                }

                Token::Identifier(s) => output.push(Token::Identifier(s.into())),

                Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Caret => {
                    // Pop ops from operator stack if
                    while let Some(from_op_stack) = operator_stack.last() {
                        // 1. it's not left parenthesis
                        if from_op_stack != &Token::LeftParen
                        // 2. op in stack has higher precedence
                            && (from_op_stack.precedence_cmp(token) == Ordering::Greater
                            // 3. same precedence and token has left assotiativity
                                || (from_op_stack.precedence_cmp(token) == Ordering::Equal
                                    && token.get_associativity() == Associativity::Left))
                        {
                            if let Some(popped) = operator_stack.pop() {
                                output.push(popped);
                            }
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(token.clone());
                }
                Token::LeftParen => operator_stack.push(Token::LeftParen),
                Token::RightParen => loop {
                    let from_op_stack = operator_stack.pop();
                    match from_op_stack {
                        Some(Token::LeftParen) => break,
                        Some(token) => output.push(match token {
                            Identifier(t) => Token::Identifier(t),
                            _ => token.clone(),
                        }),
                        None => return Err(ParseError::MissingParenthesis),
                    }
                },
                Token::Whitespace => todo!(),
                Token::Function(_) => operator_stack.push(token.clone()),
            }
        }

        // pop op stack on output
        while let Some(token) = operator_stack.pop() {
            if token == Token::LeftParen {
                return Err(ParseError::MissingParenthesis);
            }
            output.push(token.clone());
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{
        BinaryOp::{Division, Exponent},
        Value,
    };

    use super::*;
    #[test]
    fn test_set_1_rpn() {
        let parser = Parser::new();
        let input = vec![
            Token::LeftParen,
            Token::Number(8),
            Token::Slash,
            Token::Number(27),
            Token::RightParen,
            Token::Caret,
            Token::LeftParen,
            Token::Number(2),
            Token::Slash,
            Token::Number(3),
            Token::RightParen,
        ];
        let result = parser.tokens_to_reverse_polish_notation(&input);
        let answer = vec![
            Token::Number(8),
            Token::Number(27),
            Token::Slash,
            Token::Number(2),
            Token::Number(3),
            Token::Slash,
            Token::Caret,
        ];
        assert_eq!(result, Ok(answer));
    }

    #[test]
    fn test_set_1_ast() {
        let parser = Parser::new();
        let input = vec![
            Token::Number(8),
            Token::Number(27),
            Token::Slash,
            Token::Number(2),
            Token::Number(3),
            Token::Slash,
            Token::Caret,
        ];

        let result = parser.rpn_to_ast(input);
        let answer = Expr::Binary {
            lhs: Box::new(Expr::Binary {
                lhs: Box::new(Expr::Literal(Value::Integer(8))),
                op: Division,
                rhs: Box::new(Expr::Literal(Value::Integer(27))),
            }),
            op: Exponent,
            rhs: Box::new(Expr::Binary {
                lhs: Box::new(Expr::Literal(Value::Integer(2))),
                op: Division,
                rhs: Box::new(Expr::Literal(Value::Integer(3))),
            }),
        };
        assert_eq!(result, Ok(answer));
    }
}
