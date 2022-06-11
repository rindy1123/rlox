use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

use ast_printer::AstPrinter;

mod ast_printer;
mod expr;
mod lang_error;
mod parser;
mod scanner;

fn run_file(path: &Path) {
    let source = fs::read_to_string(path).unwrap();
    run(source);
}

fn run_prompt() {
    let stdin = io::stdin();
    loop {
        print!("> ");
        let mut buffer = String::new();
        io::stdout().flush().unwrap();
        let bytes = stdin.read_line(&mut buffer).unwrap();
        if bytes == 0 {
            exit(0);
        }
        run(buffer.trim().to_string());
    }
}

fn run(source: String) {
    let mut scanner = scanner::scanner::Scanner::new(source);
    scanner.scan_tokens();
    let mut parser = parser::Parser::new(scanner.tokens);
    let expression = parser.parse();

    // TODO: Error Handling
    println!("{}", AstPrinter::new().print(expression))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64)
    } else if args.len() == 2 {
        let path = Path::new(&args[1]);
        run_file(path)
    } else {
        run_prompt()
    }
}
