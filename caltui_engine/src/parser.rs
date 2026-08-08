use std::{borrow::Borrow, cmp::Ordering};

use crate::{
    ast::{self, BinaryOp, Expr},
    lexer::{
        Assosiativity,
        Token::{self, Identifier},
    },
};
pub struct OperatorTemplate {
    op: Option<Token>,
    lhs: Option<Token>,
    rhs: Option<Token>,
}
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
    pub fn new() -> Self {
        Self {}
    }
    // Build AST with Shunting yard algorithm
    pub fn build_ast(&self, tokens: Vec<Token>) -> Result<Expr, ParseError> {
        let rpn = self.tokens_to_reverse_polish_notatation(tokens)?;

        self.rpn_to_ast(rpn)
    }

    pub fn rpn_to_ast(&self, rpn: Vec<Token>) -> Result<Expr, ParseError> {
        let mut stack: Vec<Expr> = Vec::new();

        for token in rpn {
            match token {
                Token::Number(n) => {
                    stack.push(Expr::Literal(ast::Value::Number(ast::Number { value: n })));
                }

                t if t.as_binary_op().is_some() => {
                    let rhs = stack.pop().ok_or(ParseError::MissingOperand)?;
                    let lhs = stack.pop().ok_or(ParseError::MissingOperand)?;

                    stack.push(Expr::Binary {
                        lhs: Box::new(lhs),
                        op: t.as_binary_op().unwrap(),
                        rhs: Box::new(rhs),
                    });
                }
                _ => todo!(),
            }
        }

        if stack.len() != 1 {
            return Err(ParseError::InvalidExpression);
        }

        Ok(stack.pop().unwrap())
    }
    pub fn tokens_to_reverse_polish_notatation(
        &self,
        tokens: Vec<Token>,
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
                    output.push(Token::Function(ident.to_string()))
                }

                Token::Identifier(s) => output.push(Token::Identifier(s.into())),

                Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Caret => {
                    println!("Operator token found");
                    // Pop ops from operator stack if
                    while let Some(from_op_stack) = operator_stack.last() {
                        println!("while loop start");
                        // 1. it's not left parenthesis
                        if from_op_stack != &Token::LeftParen
                        // 2. op in stack has higher precedence
                            && (from_op_stack.has_greater_precedence(token) == Ordering::Greater
                            // 3. same precedence and token has left assotiativity
                                || (from_op_stack.has_greater_precedence(token) == Ordering::Equal
                                    && token.get_associativity() == Assosiativity::Left))
                        {
                            if let Some(popped) = operator_stack.pop() {
                                output.push(popped);
                            }
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(token.clone())
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
        println!("{}", operator_stack.len());
        // pop op stack on output
        while let Some(token) = operator_stack.pop() {
            if token == Token::LeftParen {
                println!("parenthesis error");
                return Err(ParseError::MissingParenthesis);
            }
            output.push(token.clone());
        }
        Ok(output)
    }
}
