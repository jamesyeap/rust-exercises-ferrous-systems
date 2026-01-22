use crate::ParseError::{InvalidExpression, InvalidNumber};

// note: enum are "sum" types - represents OR amongst its constituent types
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Square(Box<Expr>),
}

#[derive(Debug)]
pub enum ParseError {
    NoExpression,
    InvalidExpression,
    InvalidNumber,
}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut stack: Vec<Expr> = Vec::new();

    for word in input.split_ascii_whitespace() {
        match word {
            "+" => {
                let operand_1 = stack.pop().ok_or(InvalidExpression)?;
                let operand_2 = stack.pop().ok_or(InvalidExpression)?;
                stack.push(Expr::Add(Box::new(operand_1), Box::new(operand_2)));
            }

            // TODO: add more operands
            _ => {
                let number = word.parse::<i64>().map_err(|_| InvalidNumber)?;
                stack.push(Expr::Number(number));
            }
        }
    }

    assert_eq!(stack.len(), 1);

    stack.pop().ok_or(ParseError::NoExpression)
}

#[derive(Debug)]
pub enum EvalError {
    // TODO: remove this error type
    Undefined,
}

pub fn eval(expr: &Expr) -> Result<i64, EvalError> {
    match expr {
        Expr::Number(num) => Ok(*num),
        Expr::Add(operand_1, operand_2) => {
            let num_1 = eval(operand_1)?;
            let num_2 = eval(operand_2)?;
            Ok(num_1 + num_2)
        }
        _ => Err(EvalError::Undefined),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let input = "42";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn addition() {
        let input = "1 2 +";
        let expr = parse(input).unwrap();
        let value = eval(&expr).unwrap();
        assert_eq!(value, 3);
    }

    // #[test]
    // fn smoke_test() {
    //     let input = "3 sqr 4 sqr + 5 sqr -";
    //     let expr = parse(input).unwrap();
    //     let value = eval(&expr).unwrap();
    //     assert_eq!(value, 0);
    // }
}
