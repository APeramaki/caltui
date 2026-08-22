use std::ops;

#[derive(Debug, PartialEq)]
pub enum OperatorError {
    UnknownOperatorError,
    UnderflowError,
    OverflowError,
    DivisionByZero,
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

impl Number {
    fn checked_op(
        self,
        rhs: &Number,
        op: fn(u64, u64) -> Option<u64>,
        err: OperatorError,
    ) -> Result<Number, OperatorError> {
        op(self.value, rhs.value)
            .map(|value| Number { value })
            .ok_or(err)
    }
}

// Builds implementation for checked operation for Numbers
macro_rules! impl_number_op {
    ($trait:ident, $method:ident, $checked:expr, $err:expr) => {
        impl ops::$trait<Number> for Number {
            type Output = Result<Number, OperatorError>;
            fn $method(self, rhs: Number) -> Self::Output {
                self.checked_op(&rhs, $checked, $err)
            }
        }
    };
}

impl_number_op!(Add, add, u64::checked_add, OperatorError::OverflowError);
impl_number_op!(Sub, sub, u64::checked_sub, OperatorError::UnderflowError);
impl_number_op!(Mul, mul, u64::checked_mul, OperatorError::OverflowError);
impl_number_op!(Div, div, u64::checked_div, OperatorError::DivisionByZero);

// Builds implementation for operators for value
macro_rules! impl_value_op {
    ($trait:ident, $method:ident) => {
        impl ops::$trait<Value> for Value {
            type Output = Result<Value, OperatorError>;
            fn $method(self, rhs: Value) -> Self::Output {
                match (self, rhs) {
                    (Value::Number(lhs), Value::Number(rhs)) => {
                        Ok(Value::Number(ops::$trait::$method(lhs, rhs)?))
                    }
                    _ => Err(OperatorError::UnknownOperatorError),
                }
            }
        }
    };
}

impl_value_op!(Add, add);
impl_value_op!(Sub, sub);
impl_value_op!(Mul, mul);
impl_value_op!(Div, div);
