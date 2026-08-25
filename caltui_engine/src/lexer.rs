use std::cmp::Ordering;

use crate::ast::{BinaryOp, UnaryOp};

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidNumber,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),
    Identifier(String),

    Plus,
    Minus,
    Star,
    Slash,
    Caret,

    LeftParen,
    RightParen,

    Function(String),
    Whitespace,
}

#[derive(PartialEq)]
pub enum Associativity {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Binary(BinaryOp),
    Unary(UnaryOp),
}

impl Token {
    pub fn as_operator(&self) -> Option<Operator> {
        self.as_binary_op()
            .map(Operator::Binary)
            .or_else(|| self.as_unary_op().map(Operator::Unary))
    }

    fn as_unary_op(&self) -> Option<UnaryOp> {
        // To be filled
        None
    }

    fn as_binary_op(&self) -> Option<BinaryOp> {
        match self {
            Token::Caret => Some(BinaryOp::Exponent),
            Token::Plus => Some(BinaryOp::Addition),
            Token::Minus => Some(BinaryOp::Subtraction),
            Token::Star => Some(BinaryOp::Multiplication),
            Token::Slash => Some(BinaryOp::Division),
            _ => None,
        }
    }

    #[must_use]
    pub fn precedence_cmp(&self, other: &Token) -> Ordering {
        self.get_precedence().cmp(&other.get_precedence())
    }
    fn get_precedence(&self) -> i8 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Slash | Token::Star => 2,
            Token::Caret => 3,
            Token::Function(_) => 4,
            // All others listed to force adding new tokens here
            // effectively _ => 0 as these don't have precedence.
            Token::Number(_)
            | Token::Identifier(_)
            | Token::LeftParen
            | Token::RightParen
            | Token::Whitespace => 0,
        }
    }
    #[must_use]
    pub fn get_associativity(&self) -> Associativity {
        match self {
            Token::Caret => Associativity::Right,
            _ => Associativity::Left,
        }
    }
}

pub fn lexer(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let token = match c {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '^' => Token::Caret,
            x if x.is_ascii_digit() => {
                let digits: String = std::iter::once(x)
                    .chain(chars.by_ref().take_while(char::is_ascii_digit))
                    .collect();

                Token::Number(digits.parse().map_err(|_| LexerError::InvalidNumber)?)
            }
            x if x.is_whitespace() => continue,
            x => {
                let ident: String = std::iter::once(x)
                    .chain(chars.by_ref().take_while(|n| n.is_alphabetic()))
                    .collect();
                Token::Identifier(ident)
            }
        };
        tokens.push(token);
    }
    Ok(tokens)
}
