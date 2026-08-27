use std::{collections::HashMap, todo};

use crate::{
    ast::{
        Expr,
        OperatorError::{self, DivisionByZero},
        Value,
    },
    lexer::{LexerError, lexer},
    parser::{ParseError, Parser},
};

#[derive(Debug, PartialEq)]
pub enum CalcError {
    Parse(ParseError),
    Evaluation(OperatorError),
    Lexer(LexerError),
    NotANumber,
}

impl From<LexerError> for CalcError {
    fn from(err: LexerError) -> Self {
        CalcError::Lexer(err)
    }
}
impl From<ParseError> for CalcError {
    fn from(err: ParseError) -> Self {
        CalcError::Parse(err)
    }
}

impl From<OperatorError> for CalcError {
    fn from(err: OperatorError) -> Self {
        CalcError::Evaluation(err)
    }
}

pub struct Calculator {
    _variables: HashMap<String, Expr>,
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator::new()
    }
}
impl Calculator {
    #[must_use]
    pub fn new() -> Self {
        Self {
            _variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, input: &str) -> Result<Value, CalcError> {
        let parser = Parser::new();
        let r = parser.build_ast(&lexer(input)?)?;
        Ok(evaluate(r)?)
    }
}

pub fn evaluate(expr: Expr) -> Result<Value, OperatorError> {
    match expr {
        Expr::Literal(value) => Ok(value),
        Expr::Variable(_) => todo!(),
        Expr::Unary { op: _, expr: _ } => todo!(),
        Expr::Binary { lhs, op, rhs } => {
            let lhs = evaluate(*lhs)?;
            let rhs = evaluate(*rhs)?;
            match op {
                crate::ast::BinaryOp::Addition => lhs + rhs,
                crate::ast::BinaryOp::Subtraction => lhs - rhs,
                crate::ast::BinaryOp::Multiplication => lhs * rhs,
                crate::ast::BinaryOp::Division => match rhs {
                    Value::Integer(0) => Err(DivisionByZero),
                    Value::Frac(rhs) if rhs.is_zero() => Err(DivisionByZero),
                    rhs => lhs / rhs,
                },
                crate::ast::BinaryOp::Exponent => lhs.pow(rhs),
            }
        }
    }
}
