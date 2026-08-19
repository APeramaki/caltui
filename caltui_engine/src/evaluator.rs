use std::{collections::HashMap, todo};

use crate::{
    ast::{Expr, OperatorError, Value},
    lexer::lexer,
    parser::{ParseError, Parser},
};

#[derive(Debug, PartialEq)]
pub enum CalcError {
    Parse(ParseError),
    Evaluation(OperatorError),
    NotANumber,
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
    variables: HashMap<String, Expr>,
}

impl Default for Calculator {
    fn default() -> Self {
        Calculator::new()
    }
}
impl Calculator {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, input: &str) -> Result<Value, CalcError> {
        let parser = Parser::new();
        let r = parser.build_ast(lexer(input))?;
        match evaluate(r)? {
            Value::Number(value) => Ok(Value::Number(value)),
        }
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
                crate::ast::BinaryOp::Subtraction => lhs - rhs, // lhs - rhs,
                crate::ast::BinaryOp::Multiplication => lhs * rhs, // lhs * rhs,
                crate::ast::BinaryOp::Division => lhs / rhs,    // lhs / rhs,
                crate::ast::BinaryOp::Exponent => todo!(),      // lhs.pow(rhs),
            }
        }
    }
    // Err(OperatorError::UnknownOperatorError)
}
