use std::fmt;
use crate::token_form::TokenForm;

#[derive(Debug)]
pub struct Token {
    pub form: TokenForm,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

impl Token {
    pub fn eof(line: usize) -> Self {
        Self {
            form: TokenForm::Eof,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.form, self.lexeme, self.literal)
    }
}
