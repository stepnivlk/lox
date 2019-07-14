use std::{env, process};

mod token_form;
mod token;
mod scanner;
mod lox;

use lox::Lox;

fn main() {
    let sys_args: Vec<String> = env::args().collect();
    let args = &sys_args[1..];

    if args.len() > 1 {
        println!("Usage: lox [script]");
        process::exit(64);
    }

    let lox = Lox::new();

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
