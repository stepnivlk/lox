use std::{env, process};

mod ast_printer;
mod expr;
mod token_form;
mod token;
mod scanner;
mod lox;

use ast_printer::AstPrinter;
use token::{Token, Literal};
use token_form::TokenForm;
use expr::Expr;
use lox::Lox;

fn print_exprs() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token {
                form: TokenForm::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal {
                value: Some(Literal::Number(123 as f64))
            })
        }),
        operator: Token {
            form: TokenForm::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: Some(Literal::Number(45.67))
            })
        })
    };

    let ast_printer = AstPrinter::new();
    let output = ast_printer.print(&expr);
    dbg!(output);
}

fn main() {
    print_exprs();

    let sys_args: Vec<String> = env::args().collect();
    let args = &sys_args[1..];

    if args.len() > 1 {
        println!("Usage: lox [script]");
        process::exit(64);
    }

    let mut lox = Lox::new();

    let res = if args.len() == 1 {
        lox.run_file(&args[0])
    } else {
        lox.run_prompt()
    };

    match res {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(65),
    }
}
