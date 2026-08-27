use std::{num::TryFromIntError, ops, todo};

#[derive(Debug, PartialEq)]
pub enum OperatorError {
    UnknownOperatorError,
    UnderflowError,
    OverflowError,
    DivisionByZero,
    ZeroToZerothPower,
    TooLargeExponent,
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
    numerator: i64,
    denominator: i64,
    // units...
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    Frac(Rational),
    Integer(i64),
    // matrix...
}

impl Value {
    fn is_zero(self) -> bool {
        match self {
            Value::Frac(rational) if rational.is_zero() => true,
            Value::Integer(0) => true,
            _ => false,
        }
    }
    pub fn pow(self, rhs: Self) -> Result<Self, OperatorError> {
        match (self, rhs) {
            // zero to zero is undefined
            (v, t) if v.is_zero() && t.is_zero() => Err(OperatorError::ZeroToZerothPower),
            // Anything to 0th power is 1 (except above)
            (_, t) if t.is_zero() => Ok(Value::Integer(1)),

            (Value::Frac(lhs), Value::Frac(rhs)) => {
                let numerator_perfect_root =
                    perfect_root(lhs.numerator.pow(rhs.numerator as u32), rhs.denominator);
                let denominator_perfect_root =
                    perfect_root(lhs.denominator.pow(rhs.numerator as u32), rhs.denominator);
                match (numerator_perfect_root, denominator_perfect_root) {
                    (Some(numerator), Some(1)) => Ok(Value::Integer(numerator)),
                    (Some(numerator), Some(denominator)) => Ok(Value::Frac(Rational {
                        numerator,
                        denominator,
                    })),
                    (_, _) => todo!("Floating point support not yet implemented"),
                }
            }

            (Value::Frac(lhs), Value::Integer(rhs)) if rhs.is_negative() => {
                let rhs = u32::try_from(rhs.unsigned_abs())
                    .map_err(|_| OperatorError::TooLargeExponent)?;
                Ok(Value::Frac(Rational::new(
                    lhs.denominator.pow(rhs),
                    lhs.numerator.pow(rhs),
                )))
            }

            (Value::Frac(lhs), Value::Integer(rhs)) => {
                let rhs = u32::try_from(rhs).map_err(|_| OperatorError::TooLargeExponent)?;

                Ok(Value::Frac(Rational::new(
                    lhs.numerator.pow(rhs),
                    lhs.denominator.pow(rhs),
                )))
            }

            (Value::Integer(lhs), Value::Frac(rhs)) => {
                let perfect_root = perfect_root(lhs.pow(rhs.numerator as u32), rhs.denominator);
                if let Some(root) = perfect_root {
                    return Ok(Value::Integer(root));
                }
                todo!("Floating point support not yet implemented")
            }

            (Value::Integer(lhs), Value::Integer(rhs)) if rhs >= 0 => {
                let rhs = u32::try_from(rhs).map_err(|_| OperatorError::TooLargeExponent)?;
                Ok(Value::Integer(lhs.pow(rhs)))
            }

            (Value::Integer(lhs), Value::Integer(rhs)) if rhs < 0 => {
                let rhs = u32::try_from(rhs.unsigned_abs())
                    .map_err(|_| OperatorError::TooLargeExponent)?;

                Ok(Value::Frac(Rational::new(1, lhs.pow(rhs))))
            }

            (Value::Integer(_), Value::Integer(_)) => unreachable!(),
        }
    }
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
    pub fn new(numerator: i64, denominator: i64) -> Self {
        let gcd = greatest_common_divisor(numerator, denominator).cast_signed();
        // Keep the sign in the numerator so equal values compare equal.
        let sign = if denominator.is_negative() { -1 } else { 1 };

        Self {
            numerator: sign * (numerator / gcd),
            denominator: sign * (denominator / gcd),
        }
    }
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.numerator == 0
    }
    /// Collapse to an `Integer` when the fraction reduced to a whole number.
    #[must_use]
    pub fn into_value(self) -> Value {
        if self.denominator == 1 {
            Value::Integer(self.numerator)
        } else {
            Value::Frac(self)
        }
    }
}

impl ops::Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (
            self.numerator.checked_mul(rhs.numerator),
            self.denominator.checked_mul(rhs.denominator),
        ) {
            // todo: Add simplification
            (Some(numerator), Some(denominator)) => Self::new(numerator, denominator),
            _ => todo!(), // Add graceful handling for overflow
        }
    }
}

impl ops::Div for Rational {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    // Mathematically sound
    fn div(self, rhs: Self) -> Self::Output {
        self * Self {
            numerator: rhs.denominator,
            denominator: rhs.numerator,
        }
    }
}

impl ops::Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        // a / b + c / d = (ad + cb) / bd
        let ad = self.numerator.checked_mul(rhs.denominator);
        let cb = rhs.numerator.checked_mul(self.denominator);
        let numerator = match (ad, cb) {
            (Some(ad), Some(cb)) => ad.checked_add(cb),
            (_, _) => todo!(), // TODO: Need to promote!
        };

        let denominator = self.denominator.checked_mul(rhs.denominator);
        match (numerator, denominator) {
            (Some(numerator), Some(denominator)) => Self::new(numerator, denominator),
            (_, _) => todo!(), // overflow: TODO: Promote
        }
    }
}

impl ops::Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let rhs = Self::new(-rhs.numerator, rhs.denominator);
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

fn perfect_root(base: i64, exponent: i64) -> Option<i64> {
    if base == 1 {
        return Some(base);
    }

    let k = num::integer::Roots::nth_root(&base, exponent as u32);

    let exponent: Result<u32, TryFromIntError> = exponent.try_into();
    let Ok(exponent) = exponent else { return None };

    // Verify: check if k^m == n
    if k.checked_pow(exponent) == Some(base) {
        Some(k)
    } else {
        None
    }
}

macro_rules! impl_value_op {
    // Default: integer op integer stays an integer.
    ($trait:ident, $method:ident) => {
        impl_value_op!($trait, $method, |lhs: i64, rhs: i64| Ok(Value::Integer(
            ops::$trait::$method(lhs, rhs)
        )));
    };
    // Custom handling for the integer/integer case (e.g. division must not truncate).
    ($trait:ident, $method:ident, $int_int:expr) => {
        impl ops::$trait<Value> for Value {
            type Output = Result<Value, OperatorError>;
            fn $method(self, rhs: Value) -> Self::Output {
                match (self, rhs) {
                    (Value::Integer(lhs), Value::Integer(rhs)) => {
                        let int_int: fn(i64, i64) -> Self::Output = $int_int;
                        int_int(lhs, rhs)
                    }
                    (Value::Frac(lhs), Value::Frac(rhs)) => {
                        Ok(ops::$trait::$method(lhs, rhs).into_value())
                    }
                    (Value::Integer(lhs), Value::Frac(rhs)) => {
                        let lhs_as_frac = Rational::new(lhs, 1);
                        Ok(ops::$trait::$method(lhs_as_frac, rhs).into_value())
                    }
                    (Value::Frac(lhs), Value::Integer(rhs)) => {
                        let rhs_as_frac = Rational::new(rhs, 1);
                        Ok(ops::$trait::$method(lhs, rhs_as_frac).into_value())
                    } //_ =>  Err(OperatorError::UnknownOperatorError),
                }
            }
        }
    };
}

impl_value_op!(Add, add);
impl_value_op!(Sub, sub);
impl_value_op!(Mul, mul);
impl_value_op!(Div, div, |lhs: i64, rhs: i64| {
    if rhs == 0 {
        return Err(OperatorError::DivisionByZero);
    }
    Ok(Rational::new(lhs, rhs).into_value())
});
