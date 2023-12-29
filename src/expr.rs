use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    Unary(Unary<'a>),
    Binary(Binary<'a>),
    Grouping(Grouping<'a>),
}

#[derive(Debug)]
pub struct Literal<'a> {
    value: LiteralValue<'a>,
}

#[derive(Debug)]
pub enum LiteralValue<'a> {
    Number(f64),
    String(&'a str),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub struct Unary<'a> {
    operator: Token<'a>,
    right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub struct Binary<'a> {
    left: Box<Expr<'a>>,
    operator: Token<'a>,
    right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub struct Grouping<'a> {
    expr: Box<Expr<'a>>,
}

impl<'a> Literal<'a> {
    pub fn new(value: Token<'a>) -> Self {
        Literal::from(value)
    }
}

impl<'a> Unary<'a> {
    pub fn new(operator: Token<'a>, right: Expr<'a>) -> Self {
        Unary {
            operator,
            right: Box::new(right),
        }
    }
}

impl<'a> Binary<'a> {
    pub fn new(left: Expr<'a>, operator: Token<'a>, right: Expr<'a>) -> Self {
        Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl<'a> Grouping<'a> {
    pub fn new(expr: Expr<'a>) -> Self {
        Grouping {
            expr: Box::new(expr),
        }
    }
}

impl<'a> From<Token<'a>> for Literal<'a> {
    fn from(value: Token<'a>) -> Self {
        match value.kind {
            TokenKind::Number(v) => Literal {
                value: LiteralValue::Number(v),
            },
            TokenKind::String(v) => Literal {
                value: LiteralValue::String(v),
            },
            TokenKind::True => Literal {
                value: LiteralValue::Boolean(true),
            },
            TokenKind::False => Literal {
                value: LiteralValue::Boolean(false),
            },
            TokenKind::Nil => Literal {
                value: LiteralValue::Nil,
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> std::fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary(v) => write!(f, "{}", v),
            Expr::Literal(v) => write!(f, "{}", v),
            Expr::Grouping(v) => write!(f, "{}", v),
            Expr::Unary(v) => write!(f, "{}", v),
        }
    }
}

impl<'a> std::fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            LiteralValue::String(v) => write!(f, "{}", v),
            LiteralValue::Boolean(v) => match v {
                true => write!(f, "true"),
                false => write!(f, "false"),
            },
            LiteralValue::Number(v) => write!(f, "{}", v),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

impl<'a> std::fmt::Display for Unary<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.operator, self.right)
    }
}

impl<'a> std::fmt::Display for Binary<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.operator, self.left, self.right)
    }
}

impl<'a> std::fmt::Display for Grouping<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.expr)
    }
}
