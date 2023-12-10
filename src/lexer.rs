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
    pub fn new(src: &str) -> Self {
        Lexer {
            src,
            chars: src.char_indices().collect(),
            start: 0,
            current: 0,
            end: 0,
        }
    }

    fn scan_token(&self) -> Option<Token> {
        None
    }

    fn scan_token_kind(&self) -> TokenKind {
        match self.advance() {
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
        }
    }

    fn take_select(
        &mut self,
        expected: char,
        kind_true: TokenKind,
        kind_false: TokenKind,
    ) -> TokenKind {
        if self.peek(0) == expected {
            kind_true
        } else {
            kind_false
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek(0) == '\0'
    }

    fn peek(&self, offset: usize) -> char {
        self.chars
            .get(self.current + offset)
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
