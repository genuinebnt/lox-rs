use crate::{expr::*, token::TokenKind};

pub enum Output<'a> {
    Number(f64),
    Boolean(bool),
    String(&'a str),
    Nil,
}

pub enum RuntimeError {
    LiteralError,
    UnaryError,
    BinaryError,
}

pub struct Interpreter;

impl<'a> Interpreter {
    pub fn interpret(&self, expr: Expr<'a>) -> Result<Output<'a>, RuntimeError> {
        match expr {
            Expr::Literal(v) => self.evaluate_literal(v),
            Expr::Binary(v) => self.evaluate_binary(v),
            Expr::Unary(v) => self.evaluate_unary(v),
            Expr::Grouping(v) => self.evaluate_grouping(v),
            Expr::Ternary(v) => self.evaluate_ternary(v),
        }
    }

    fn evaluate_literal(&mut self, expr: Literal<'a>) -> Result<Output<'a>, RuntimeError> {
        let expr = match expr.value {
            LiteralValue::Number(v) => Output::Number(v),
            LiteralValue::Boolean(v) => Output::Boolean(v),
            LiteralValue::String(v) => Output::String(v),
            LiteralValue::Nil => Output::Nil,
        };

        Ok(expr)
    }

    fn evaluate_grouping(&mut self, expr: Grouping<'a>) -> Result<Output<'a>, RuntimeError> {
        self.interpret(*expr.expr)
    }

    fn evaluate_unary(&mut self, expr: Unary<'a>) -> Result<Output<'a>, RuntimeError> {
        let right = self.interpret(*expr.right)?;

        match right {
            Output::Number(v) => {
                if let TokenKind::Minus = expr.operator.kind {
                    Ok(Output::Number(-v))
                } else {
                    Ok(Output::Boolean(false))
                }
            }
            Output::Boolean(v) => match v {
                true => Ok(Output::Boolean(false)),
                false => Ok(Output::Boolean(true)),
            },
            Output::String(_) => {
                if let TokenKind::Minus = expr.operator.kind {
                    Err(RuntimeError::UnaryError)
                } else {
                    Ok(Output::Boolean(true))
                }
            }
            Output::Nil => {
                if let TokenKind::Minus = expr.operator.kind {
                    Ok(Output::Nil)
                } else {
                    Ok(Output::Boolean(true))
                }
            }
        }
    }

    fn evaluate_binary(&mut self, expr: Binary<'a>) -> Result<Output<'a>, RuntimeError> {
        let left = self.interpret(*expr.left)?;
        let right = self.interpret(*expr.right)?;
    }
}
