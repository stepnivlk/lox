use std::fs;
use std::io::{stdin, stdout, Write};

use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }
    
    pub fn run_file(&mut self, path: &String) -> Result<(), ()> {
        let program: String = fs::read_to_string(path).unwrap().parse().unwrap();
        self.run(program);

        if self.had_error {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn run_prompt(&mut self) -> Result<(), ()> {
        loop {
            let mut line = String::new();

            print!(">>> ");

            let _ = stdout().flush();

            stdin().read_line(&mut line).unwrap();

            match line.chars().next_back() {
                Some('\n') => line.pop(),
                Some('\r') => line.pop(),
                _ => None
            };

            self.run(line);
        }
    }

    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(self, source);

        let tokens = scanner.scan_tokens();

        for token in tokens {
            dbg!(token);
        }
    }

    fn report(&mut self, line: usize, spot: String, message: String) {
        eprintln!("[line: {}] Error {}: {}", line, spot, message);
        self.had_error = true;
    }
}
