use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    Unary(Unary<'a>),
    Binary(Binary<'a>),
    Grouping(Grouping<'a>),
    Ternary(Ternary<'a>),
}

#[derive(Debug)]
pub struct Literal<'a> {
    pub value: LiteralValue<'a>,
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
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub struct Binary<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub struct Ternary<'a> {
    pub left: Box<Expr<'a>>,
    pub left_operator: Token<'a>,
    pub middle: Box<Expr<'a>>,
    pub right_operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

#[derive(Debug)]
pub struct Grouping<'a> {
    pub expr: Box<Expr<'a>>,
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

impl<'a> Ternary<'a> {
    pub fn new(
        left: Expr<'a>,
        left_operator: Token<'a>,
        middle: Expr<'a>,
        right_operator: Token<'a>,
        right: Expr<'a>,
    ) -> Self {
        Ternary {
            left: Box::new(left),
            left_operator,
            middle: Box::new(middle),
            right_operator,
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
            Expr::Ternary(v) => write!(f, "{}", v),
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
        write!(f, "(group {})", self.expr)
    }
}

impl<'a> std::fmt::Display for Ternary<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {} {} {})",
            self.left, self.left_operator, self.middle, self.right_operator, self.right
        )
    }
}
