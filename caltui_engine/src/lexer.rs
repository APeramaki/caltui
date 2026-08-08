use std::cmp::Ordering;

use crate::ast::BinaryOp;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(u64),
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

impl Token {
    pub fn as_binary_op(&self) -> Option<BinaryOp> {
        match self {
            Token::Caret => Some(BinaryOp::Exponent),
            Token::Plus => Some(BinaryOp::Addition),
            Token::Minus => Some(BinaryOp::Subtraction),
            Token::Star => Some(BinaryOp::Multiplication),
            Token::Slash => Some(BinaryOp::Division),
            _ => None,
        }
    }

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
    pub fn get_associativity(&self) -> Associativity {
        match self {
            Token::Caret => Associativity::Right,
            _ => Associativity::Left,
        }
    }
}

pub fn lexer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    // let mut current = "";

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
                let mut number: u64 = x.to_digit(10).unwrap_or(0).into();
                while chars.peek().is_some_and(|n| n.is_ascii_digit()) {
                    number = number * 10 + chars.next().unwrap().to_digit(10).unwrap_or(0) as u64;
                }
                Token::Number(number)
            }
            x if x.is_whitespace() => continue,
            x => {
                let mut ident = String::from(x);
                while chars.peek().is_some_and(|n| n.is_alphabetic()) {
                    ident.push(chars.next().unwrap());
                }
                Token::Identifier(ident)
            }
        };
        tokens.push(token);
    }
    tokens
}
