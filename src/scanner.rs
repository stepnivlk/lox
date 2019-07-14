use crate::lox::Lox;
use crate::token::Token;
use crate::token_form::TokenForm;

pub struct Scanner<'a> {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    lox: &'a mut Lox,
}

impl<'a> Scanner<'a> {
    pub fn new(lox: &'a mut Lox, source: String) -> Self {
        Scanner {
            source, start: 0, current: 0, line: 1, tokens: vec![], lox
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_the_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::eof(self.line));

        vec![]
    }

    fn scan_token(&mut self) {
        let ch = self.advance();

        match ch {
            Some('(') => self.add_token(TokenForm::LeftParen),
            Some(')') => self.add_token(TokenForm::RightParen),
            Some('{') => self.add_token(TokenForm::LeftBrace),
            Some('}') => self.add_token(TokenForm::RightBrace),
            Some(',') => self.add_token(TokenForm::Comma),
            Some('.') => self.add_token(TokenForm::Dot),
            Some('-') => self.add_token(TokenForm::Minus),
            Some('+') => self.add_token(TokenForm::Plus),
            Some(';') => self.add_token(TokenForm::Semicolon),
            Some('*') => self.add_token(TokenForm::Star),
            Some('!') => self.add_token(
                if self.is_match('=') {
                    TokenForm::BangEqual
                } else {
                    TokenForm::Bang
                }
            ),
            Some('=') => self.add_token(
                if self.is_match('=') {
                    TokenForm::EqualEqual
                } else {
                    TokenForm::Equal
                }
            ),
            Some('<') => self.add_token(
                if self.is_match('=') {
                    TokenForm::LessEqual
                } else {
                    TokenForm::Less
                }
            ),
            Some('>') => self.add_token(
                if self.is_match('=') {
                    TokenForm::GreaterEqual
                } else {
                    TokenForm::Greater
                }
            ),

            Some(_) => self.lox.error(self.line, "Unexpected character".to_string()),
            None => (),
        }
    }

    fn is_match(&self, expected: char) -> bool {
        if self.is_at_the_end() {
            return false
        }

        match self.source.chars().nth(self.current) {
            Some(ch) => {
                if ch == expected {
                    self.current = self.current + 1;
                    true
                } else {
                    false
                }
            },
            None => false 
        }
    }

    fn add_token(&mut self, form: TokenForm) {
        let text = &self.source[self.start..self.current];

        let token = Token {
            form,
            lexeme: text.to_string(),
            literal: "".to_string(),
            line: self.line,
        };

        self.tokens.push(token);
    }

    // TODO
    fn advance(&mut self) -> Option<char> {
        self.current = self.current + 1;
        self.source.chars().nth(self.current - 1)
    }

    fn is_at_the_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
