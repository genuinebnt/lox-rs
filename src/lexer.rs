use crate::token::*;
use std::collections::HashMap;
use TokenKind::*;

pub struct Lexer<'a> {
    src: &'a str,
    chars: Vec<(usize, char)>,
    start: usize,
    current: usize,
    end: usize,
    line: usize,
    keywords: HashMap<&'a str, TokenKind<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let keywords = Self::create_keyword_map();

        Lexer {
            src,
            chars: src.char_indices().collect(),
            start: 0,
            current: 0,
            end: 0,
            line: 0,
            keywords,
        }
    }

    fn create_keyword_map() -> HashMap<&'a str, TokenKind<'a>> {
        let mut keywords = HashMap::new();

        keywords.insert("and", And);
        keywords.insert("class", Class);
        keywords.insert("else", Else);
        keywords.insert("false", False);
        keywords.insert("fun", Fun);
        keywords.insert("for", For);
        keywords.insert("if", If);
        keywords.insert("nil", Nil);
        keywords.insert("or", Or);
        keywords.insert("print", Print);
        keywords.insert("return", Return);
        keywords.insert("super", Super);
        keywords.insert("this", This);
        keywords.insert("true", True);
        keywords.insert("var", Var);
        keywords.insert("while", While);

        keywords
    }

    fn scan_token(&mut self) -> Option<Token<'a>> {
        let kind = self.scan_token_kind();
        let span = Span::new(self.start, self.end);

        if kind == Eof {
            None
        } else {
            Some(Token::new(kind, span))
        }
    }

    fn scan_token_kind(&mut self) -> TokenKind<'a> {
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
            '?' => QuestionMark,
            ':' => Colon,
            '!' => self.take_select('=', BangEqual, Bang),
            '=' => self.take_select('=', EqualEqual, Equal),
            '>' => self.take_select('=', GreaterEqual, Greater),
            '<' => self.take_select('=', LessEqual, Less),
            '/' => self.comment_or(Slash),
            '"' => self.string(),
            '\n' => {
                self.line += 1;
                Skip(ch)
            }
            '\r' | '\t' | ' ' => Skip(ch),
            '\0' => Eof,
            ch if ch.is_digit(10) => self.take_number(),
            ch if ch.is_alphanumeric() => self.take_identifier_or_keyword(),
            _ => Error("a"),
        }
    }

    fn take_select(
        &mut self,
        expected: char,
        kind_true: TokenKind<'a>,
        kind_false: TokenKind<'a>,
    ) -> TokenKind<'a> {
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

    fn advance_by(&mut self, value: i32) {
        for _ in 0..=value {
            self.advance();
        }
    }

    fn comment_or(&mut self, or: TokenKind<'a>) -> TokenKind<'a> {
        if self.take('/') {
            let comment = self.take_single_line_comment();
            Comment(comment)
        } else if self.take('*') {
            let comment = self.take_multi_line_comment();
            Comment(comment)
        } else {
            or
        }
    }

    fn string(&mut self) -> TokenKind<'a> {
        while self.peek(0) != '"' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return TokenKind::Error("Unterminated String");
        }
        self.advance();
        self.end = self.current;

        let value = &self.src[self.start + 1..self.end - 1];
        TokenKind::String(value)
    }

    fn take_single_line_comment(&mut self) -> &'a str {
        while self.peek(0) != '\n' && !self.is_at_end() {
            self.advance();
        }
        self.end = self.current;

        &self.src[self.start + 2..self.end]
    }

    fn take_multi_line_comment(&mut self) -> &'a str {
        while self.peek(0) != '*' && self.peek(1) != '/' && !self.is_at_end() {
            if self.peek(0) == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        self.end = self.current;
        self.advance_by(2);

        &self.src[self.start + 2..self.end]
    }

    fn take_number(&mut self) -> TokenKind<'a> {
        while self.peek(0).is_digit(10) {
            self.advance();
        }

        if self.peek(0) == '.' && self.peek(1).is_digit(10) {
            self.advance();
        }

        while self.peek(0).is_digit(10) {
            self.advance();
        }

        self.end = self.current - 1;

        let value = self.src[self.start..=self.end].parse::<f64>().unwrap();
        Number(value)
    }

    fn take_identifier_or_keyword(&mut self) -> TokenKind<'a> {
        while self.peek(0).is_alphanumeric() {
            self.advance();
        }

        self.end = self.current;
        let value = &self.src[self.start..self.end];
        self.keywords
            .get(&value)
            .clone()
            .unwrap_or(&Identifier(value))
            .clone()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

#[cfg(test)]

fn print_token<'a>(tokens: impl Iterator<Item = Token<'a>>) {
    println!("{:?}", tokens.collect::<Vec<Token>>());
}

#[test]
#[ignore]
fn test_lexer() {
    let contents = std::fs::read_to_string("src/content.txt").unwrap();

    let lexer = Lexer::new(&contents);

    print_token(lexer);
}
