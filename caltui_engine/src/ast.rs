use std::{ops, println, todo};

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rational {
    pub numerator: i64,
    pub denumerator: i64,
    // units...
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Frac(Rational),
    Integer(i64),
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

impl Rational {
    #[must_use]
    pub fn new(numerator: i64, denumerator: i64) -> Self {
        let gcd = greatest_common_divisor(numerator, denumerator) as i64;
        println!("NEW: numerator: {numerator}, denumerator: {denumerator}, gcd: {gcd}");
        Self {
            numerator: numerator / gcd,
            denumerator: denumerator / gcd,
        }
    }
}

impl ops::Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (
            self.numerator.checked_mul(rhs.numerator),
            self.denumerator.checked_mul(rhs.denumerator),
        ) {
            // todo: Add simplification
            (Some(numerator), Some(denumerator)) => Self::new(numerator, denumerator),
            _ => todo!(), // Add graceful handling for overflow
        }
    }
}

impl ops::Div for Rational {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * Self {
            numerator: rhs.denumerator,
            denumerator: rhs.numerator,
        }
    }
}

impl ops::Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        // a / b + c / d = (ad + cb) / bd
        let ad = self.numerator.checked_mul(rhs.denumerator);
        let cb = rhs.numerator.checked_mul(self.denumerator);
        let numerator = match (ad, cb) {
            (Some(ad), Some(cb)) => ad.checked_add(cb),
            (_, _) => todo!(), // TODO: Need to promote!
        };

        let denumerator = self.denumerator.checked_mul(rhs.denumerator);
        match (numerator, denumerator) {
            (Some(numerator), Some(denumerator)) => Self::new(numerator, denumerator),
            (_, _) => todo!(), // overflow: TODO: Promote
        }
    }
}

impl ops::Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let rhs = Self::new(-rhs.numerator, rhs.denumerator);
        self + rhs
    }
}

// Find largest common divisor. Euclidian algorithm
fn greatest_common_divisor(lhs: i64, rhs: i64) -> u64 {
    let mut a = lhs.unsigned_abs();
    let mut b = rhs.unsigned_abs();

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

macro_rules! impl_value_op {
    ($trait:ident, $method:ident) => {
        impl ops::$trait<Value> for Value {
            type Output = Result<Value, OperatorError>;
            fn $method(self, rhs: Value) -> Self::Output {
                match (self, rhs) {
                    (Value::Integer(lhs), Value::Integer(rhs)) => {
                        Ok(Value::Integer(ops::$trait::$method(lhs, rhs)))
                    }
                    (Value::Frac(lhs), Value::Frac(rhs)) => {
                        Ok(Value::Frac(ops::$trait::$method(lhs, rhs)))
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
