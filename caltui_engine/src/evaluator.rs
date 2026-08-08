use std::collections::HashMap;

use crate::ast::{Expr, Number};

#[derive(Debug, PartialEq)]
pub enum CalcError {
    UnknownError,
    ParseIntError,
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

    pub fn execute(&mut self, input: &str) -> Result<Number, CalcError> {
        input
            .parse()
            .map(|x| Number { value: x })
            .map_err(|_| CalcError::ParseIntError)
    }
}
