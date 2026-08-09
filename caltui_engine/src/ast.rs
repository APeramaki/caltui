use std::ops;

#[derive(Debug, PartialEq)]
pub enum OperatorError {
    UnknownOperatorError
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Root,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponent,
}

#[derive(Debug, PartialEq)]
pub struct Number {
    pub value: u64,
    // units...
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(Number),
    // matrix...
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Value),

    Variable(String),

    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },

    Binary {
        lhs: Box<Expr>,
        op: BinaryOp,
        rhs: Box<Expr>,
    },
}

impl ops::Add<Value> for Value {
    type Output = Result<Value, OperatorError>;
    fn add(self, rhs: Value) -> Result<Value, OperatorError> {
        match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs + rhs)),
            // more to follow...
            _ => Err(OperatorError::UnknownOperatorError)
        }
    }
}

impl ops::Add<Number> for Number {
    type Output= Number;
    fn add(self, rhs: Number) -> Number{
        Number { value : self.value + rhs.value}
        
    }
}
