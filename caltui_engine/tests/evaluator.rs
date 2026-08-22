use caltui_engine::ast::Expr::Literal;
use caltui_engine::ast::{
    BinaryOp::Addition,
    Number, Value,
};


#[cfg(test)]
mod tests {

    use std::println;

    use caltui_engine::{
        ast::Expr,
        evaluator::evaluate,
    };

    use super::*;

    #[test]
    fn evaluate_sum() {
        let input = Expr::Binary {
            lhs: Box::new(Expr::Binary {
                lhs: Box::new(Literal(Value::Number(Number { value: 3 }))),
                op: Addition,
                rhs: Box::new(Literal(Value::Number(Number { value: 4 }))),
            }),
            op: Addition,
            rhs: Box::new(Expr::Binary {
                lhs: Box::new(Literal(Value::Number(Number { value: 5 }))),
                op: Addition,
                rhs: Box::new(Literal(Value::Number(Number { value: 6 }))),
            }),
        };
        let output = evaluate(input).ok().unwrap();
        println!("{:#?}", output);
        let answer = Value::Number(Number { value: 18 });
        assert_eq!(output, answer);
    }
}
