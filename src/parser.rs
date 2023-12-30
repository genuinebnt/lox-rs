use crate::{
    expr::*,
    token::{
        Token,
        TokenKind::{self, *},
    },
};

#[derive(Debug)]
pub enum ParserError<'a> {
    Eof,
    UnexpectedToken(Token<'a>),
    UnmatchedToken,
}

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: impl Iterator<Item = Token<'a>>) -> Parser<'a> {
        Parser {
            tokens: tokens
                .filter(|t| !(matches!(t.kind, TokenKind::Skip(_))))
                .collect(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr<'a>, ParserError<'a>> {
        self.comma()
    }

    pub fn comma(&mut self) -> Result<Expr<'a>, ParserError<'a>> {
        let mut expr = self.expression()?;

        while let Comma = self.peek(0).kind {
            let operator = self.advance();
            let right = self.expression()?;
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr<'a>, ParserError<'a>> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr<'a>, ParserError<'a>> {
        let mut expr: Expr = self.comparison()?;

        while let BangEqual | EqualEqual = self.peek(0).kind {
            let operator = self.advance();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(Binary::new(expr, operator, right))
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr<'a>, ParserError<'a>> {
        let mut expr = self.term()?;

        while let Greater | GreaterEqual | Less | LessEqual = self.peek(0).kind {
            let operator = self.advance();
            let right = self.term()?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr<'a>, ParserError<'a>> {
        let mut expr = self.factor()?;

        while let Minus | Plus = self.peek(0).kind {
            let operator = self.advance();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr<'a>, ParserError<'a>> {
        let mut expr = self.unary()?;

        while let Slash | Star = self.peek(0).kind {
            let operator = self.advance();
            let right = self.unary()?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr<'a>, ParserError<'a>> {
        if let Bang | Minus = self.peek(0).kind {
            let operator = self.advance();
            let right = self.unary()?;
            Ok(Expr::Unary(Unary::new(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr<'a>, ParserError<'a>> {
        let current = self.advance();

        match current.kind {
            False | True | Nil => Ok(Expr::Literal(Literal::new(current))),
            Number(_) | String(_) => Ok(Expr::Literal(Literal::new(current))),
            LeftParen => {
                let expr = self.expression()?;
                self.consume(RightParen)?;
                Ok(Expr::Grouping(Grouping::new(expr)))
            }
            _ => Err(ParserError::UnexpectedToken(current)),
        }
    }

    fn advance(&mut self) -> Token<'a> {
        if self.is_at_end() {
            self.peek(0)
        } else {
            let current = self.peek(0);
            self.current += 1;
            current
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.tokens.len() - 1
    }

    fn peek(&self, pos: i32) -> Token<'a> {
        let index = self.current + pos as usize;

        self.tokens
            .get(index)
            .copied()
            .expect(format!("index {}", index).as_str())
    }

    fn consume(&mut self, kind: TokenKind) -> Result<Token<'a>, ParserError<'a>> {
        let current = self.peek(0);

        if current.kind == TokenKind::Eof {
            Err(ParserError::Eof)
        } else if current.kind != kind {
            Err(ParserError::UnexpectedToken(current))
        } else {
            Ok(self.advance())
        }
    }

    fn synchronize(&mut self) {
        use TokenKind::*;
        while !self.is_at_end() {
            let current = self.advance();

            if let Class | Fun | Var | For | If | While | Print | Return | SemiColon = current.kind
            {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn test_parser() {
        let contents = std::fs::read_to_string("src/content.txt").unwrap();
        let lexer = Lexer::new(&contents);

        let mut parser = Parser::new(lexer);
        match parser.parse() {
            Ok(v) => println!("{:}", v),
            Err(e) => println!("{:?}", e),
        }
    }
}
