use crate::token::*;
use TokenKind::*;

pub struct Lexer<'a> {
    src: &'a str,
    chars: Vec<(usize, char)>,
    start: usize,
    current: usize,
    end: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src,
            chars: src.char_indices().collect(),
            start: 0,
            current: 0,
            end: 0,
        }
    }

    fn scan_token(&mut self) -> Option<Token> {
        let kind = self.scan_token_kind();
        let span = Span::new(self.start, self.end);

        if kind == Eof {
            None
        } else {
            Some(Token::new(kind, span))
        }
    }

    fn scan_token_kind(&mut self) -> TokenKind {
        self.start = self.current;
        self.end = self.current;

        let ch = self.advance();
        match ch {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ',' => Comma,
            '.' => Dot,
            '-' => Minus,
            '+' => Plus,
            ';' => SemiColon,
            '*' => Star,
            '!' => self.take_select('=', BangEqual, Bang),
            '=' => self.take_select('=', EqualEqual, Equal),
            '>' => self.take_select('=', GreaterEqual, Greater),
            '<' => self.take_select('=', LessEqual, Less),
            '\0' => Eof,
            _ => Unimplemented(ch),
        }
    }

    fn take_select(
        &mut self,
        expected: char,
        kind_true: TokenKind,
        kind_false: TokenKind,
    ) -> TokenKind {
        let kind = match self.take(expected) {
            true => kind_true,
            false => kind_false,
        };

        self.end = self.current - 1;
        kind
    }

    fn take(&mut self, expected: char) -> bool {
        if self.peek(0) == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek(0) == '\0'
    }

    fn peek(&self, offset: i32) -> char {
        self.chars
            .get(self.current + offset as usize)
            .copied()
            .unwrap_or((self.src.len(), '\0'))
            .1
    }

    fn advance(&mut self) -> char {
        let ch = self.peek(0);
        self.current += 1;
        ch
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

#[cfg(test)]
#[test]
fn test_scan_token() {
    let src = "{}(),.-+;*!!====>>=<<=";

    let lexer = Lexer::new(src);
    let tokens: Vec<_> = lexer.collect();

    println!("{:?}", tokens);
}
