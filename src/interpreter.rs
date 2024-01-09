use crate::{expr::*, token::TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Output {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

impl From<Output> for bool {
    fn from(value: Output) -> Self {
        match value {
            Output::Nil => false,
            Output::Boolean(v) => v,
            _ => true,
        }
    }
}

pub enum RuntimeError {
    InvalidOperand,
    DivisionByZero,
    InvalidOperation,
}

pub struct Interpreter;

impl<'a> Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&mut self, expr: Expr<'a>) -> Result<Output, RuntimeError> {
        match expr {
            Expr::Literal(v) => self.evaluate_literal(v),
            Expr::Binary(v) => self.evaluate_binary(v),
            Expr::Unary(v) => self.evaluate_unary(v),
            Expr::Grouping(v) => self.evaluate_grouping(v),
            Expr::Ternary(v) => self.evaluate_ternary(v),
        }
    }

    fn evaluate_literal(&mut self, expr: Literal<'a>) -> Result<Output, RuntimeError> {
        let expr = match expr.value {
            LiteralValue::Number(v) => Output::Number(v),
            LiteralValue::Boolean(v) => Output::Boolean(v),
            LiteralValue::String(v) => Output::String(v.to_owned()),
            LiteralValue::Nil => Output::Nil,
        };

        Ok(expr)
    }

    fn evaluate_grouping(&mut self, expr: Grouping<'a>) -> Result<Output, RuntimeError> {
        self.interpret(*expr.expr)
    }

    fn evaluate_unary(&mut self, expr: Unary<'a>) -> Result<Output, RuntimeError> {
        let right = self.interpret(*expr.right)?;

        let is_truthy = bool::from(right.clone());

        match expr.operator.kind {
            TokenKind::Minus => match right {
                Output::Number(v) => Ok(Output::Number(-v)),
                _ => Err(RuntimeError::InvalidOperand),
            },
            TokenKind::Bang => Ok(Output::Boolean(!is_truthy)),
            _ => unreachable!("Invalid token"),
        }
    }

    fn evaluate_binary(&mut self, expr: Binary<'a>) -> Result<Output, RuntimeError> {
        let left = self.interpret(*expr.left)?;
        let right = self.interpret(*expr.right)?;

        match expr.operator.kind {
            TokenKind::Minus => match (left, right) {
                (Output::Number(l), Output::Number(r)) => Ok(Output::Number(l + r)),
                _ => Err(RuntimeError::InvalidOperand),
            },
            TokenKind::Slash => match (left, right) {
                (Output::Number(l), Output::Number(r)) => {
                    if r == 0.0 {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(Output::Number(l / r))
                    }
                }
                _ => Err(RuntimeError::InvalidOperand),
            },
            TokenKind::Star => match (left, right) {
                (Output::Number(l), Output::Number(r)) => Ok(Output::Number(l * r)),
                _ => Err(RuntimeError::InvalidOperand),
            },
            TokenKind::Plus => match (left, right) {
                (Output::Number(l), Output::Number(r)) => Ok(Output::Number(l + r)),
                (Output::String(l), Output::String(r)) => Ok(Output::String(format!("{}{}", l, r))),
                _ => Err(RuntimeError::InvalidOperand),
            },
            TokenKind::Greater => match (left, right) {
                (Output::Number(l), Output::Number(r)) => Ok(Output::Boolean(l > r)),
                _ => Err(RuntimeError::InvalidOperand),
            },
            TokenKind::GreaterEqual => match (left, right) {
                (Output::Number(l), Output::Number(r)) => Ok(Output::Boolean(l >= r)),
                _ => Err(RuntimeError::InvalidOperand),
            },
            TokenKind::Less => match (left, right) {
                (Output::Number(l), Output::Number(r)) => Ok(Output::Boolean(l < r)),
                _ => Err(RuntimeError::InvalidOperand),
            },
            TokenKind::LessEqual => match (left, right) {
                (Output::Number(l), Output::Number(r)) => Ok(Output::Boolean(l <= r)),
                _ => Err(RuntimeError::InvalidOperand),
            },
            TokenKind::BangEqual => Ok(Output::Boolean(left != right)),
            TokenKind::EqualEqual => Ok(Output::Boolean(left == right)),
            _ => unreachable!("Unreachable code"),
        }
    }

    fn evaluate_ternary(&mut self, expr: Ternary<'a>) -> Result<Output, RuntimeError> {
        let left = self.interpret(*expr.left)?;
        let middle = self.interpret(*expr.middle)?;
        let right = self.interpret(*expr.right)?;

        match expr.left_operator.kind {
            TokenKind::QuestionMark => match expr.right_operator.kind {
                TokenKind::Colon => match left {
                    Output::Boolean(true) => Ok(middle),
                    Output::Boolean(false) => Ok(right),
                    _ => Err(RuntimeError::InvalidOperation),
                },
                _ => unreachable!("Unreachable code"),
            },
            _ => unreachable!("Unreachable code"),
        }
    }
}
