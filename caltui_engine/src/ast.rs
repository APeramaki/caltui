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
