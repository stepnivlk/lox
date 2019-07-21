use crate::lox::Lox;
use crate::token::{Token, Literal};
use crate::token_form::TokenForm;

pub struct Scanner<'a> {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    lox: &'a mut Lox,
}

// TODO: Extract logic, remove Lox dependency & better scanning through source.
impl<'a> Scanner<'a> {
    pub fn new(lox: &'a mut Lox, source: String) -> Self {
        Self {
            source, start: 0, current: 0, line: 1, tokens: vec![], lox
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_the_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::eof(self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let ch = self.advance();

        match ch {
            Some('(') => self.add_token(TokenForm::LeftParen, None),
            Some(')') => self.add_token(TokenForm::RightParen, None),
            Some('{') => self.add_token(TokenForm::LeftBrace, None),
            Some('}') => self.add_token(TokenForm::RightBrace, None),
            Some(',') => self.add_token(TokenForm::Comma, None),
            Some('.') => self.add_token(TokenForm::Dot, None),
            Some('-') => self.add_token(TokenForm::Minus, None),
            Some('+') => self.add_token(TokenForm::Plus, None),
            Some(';') => self.add_token(TokenForm::Semicolon, None),
            Some('*') => self.add_token(TokenForm::Star, None),
            Some('!') => {
                let token = if self.matches('=') {
                    TokenForm::BangEqual
                } else {
                    TokenForm::Bang
                };

                self.add_token(token, None);
            },
            Some('=') => {
                let token = if self.matches('=') {
                    TokenForm::EqualEqual
                } else {
                    TokenForm::Equal
                };

                self.add_token(token, None);
            },
            Some('<') => {
                let token = if self.matches('=') {
                    TokenForm::LessEqual
                } else {
                    TokenForm::Less
                };

                self.add_token(token, None);
            },
            Some('>') => {
                let token = if self.matches('=') {
                    TokenForm::GreaterEqual
                } else {
                    TokenForm::Greater
                };

                self.add_token(token, None);
            },
            Some('/') => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_the_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenForm::Slash, None)
                }
            },

            Some(' ') => (),
            Some('\r') => (),
            Some('\t') => (),

            Some('\n') => self.line += 1,

            Some('"') => self.string(),

            Some(c) if self.is_digit(c) => self.number(),

            Some(c) if self.is_alpha(c) => self.identifier(),

            Some(_) => self.lox.error(self.line, "Unexpected character".to_string()),
            None => (),
        }
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let form =  match &self.source[self.start..self.current] {
            "and" => TokenForm::And,
            "class" => TokenForm::Class,
            "else" => TokenForm::Else,
            "false" => TokenForm::False,
            "for" => TokenForm::For,
            "fun" => TokenForm::Fun,
            "if" => TokenForm::If,
            "nil" => TokenForm::Nil,
            "or" => TokenForm::Or,
            "print" => TokenForm::Print,
            "return" => TokenForm::Return,
            "super" => TokenForm::Super,
            "this" => TokenForm::This,
            "true" => TokenForm::True,
            "var" => TokenForm::Var,
            "while" => TokenForm::While,
            _ => TokenForm::Identifier,
        };

        self.add_token(form, None);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current];
        let value = value.parse::<f64>().unwrap();

        self.add_token(TokenForm::Number, Some(Literal::Number(value)));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0'
        }

        match self.source.chars().nth(self.current + 1) {
            Some(ch) => ch,
            None => '\n',
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_the_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_the_end() {
            self.lox.error(self.line, "Unterminated string".to_string());
            return
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        let value = Literal::Str(value.to_string());

        self.add_token(TokenForm::String, Some(value));
    }

    fn peek(&self) -> char {
        if self.is_at_the_end() {
            return '\0'
        }

        match self.source.chars().nth(self.current) {
            Some(ch) => ch,
            None => '\0'
        }
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_the_end() {
            return false
        }

        match self.source.chars().nth(self.current) {
            Some(ch) => {
                if ch == expected {
                    self.current += 1;
                    true
                } else {
                    false
                }
            },
            None => false 
        }
    }

    fn add_token(&mut self, form: TokenForm, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];

        let token = Token {
            form,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        };

        self.tokens.push(token);
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn is_at_the_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
