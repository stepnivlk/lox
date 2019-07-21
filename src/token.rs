use std::fmt;
use crate::token_form::TokenForm;

#[derive(Debug)]
pub enum Literal {
    Str(String),
    Number(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Str(string) => write!(f, "{}", string),
            Literal::Number(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub form: TokenForm,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn eof(line: usize) -> Self {
        Self {
            form: TokenForm::Eof,
            lexeme: "".to_string(),
            literal: None, 
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.form, self.lexeme)
    }
}
