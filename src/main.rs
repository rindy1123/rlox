use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

use interpreter::Interpreter;
use lang_error::LangError;
use resolver::Resolver;

mod environment;
mod expr;
mod interpreter;
mod lang_error;
mod object;
mod parser;
mod resolver;
mod scanner;
mod stmt;

fn run_file(path: &Path, interpreter: &mut Interpreter) {
    let source = fs::read_to_string(path).unwrap();
    if let Err(e) = run(source, interpreter) {
        match e {
            LangError::RuntimeError { .. } => exit(70),
            _ => exit(65),
        }
    };
}

fn run_prompt(interpreter: &mut Interpreter) {
    let stdin = io::stdin();
    loop {
        print!("> ");
        let mut buffer = String::new();
        io::stdout().flush().unwrap();
        let bytes = stdin.read_line(&mut buffer).unwrap();
        if bytes == 0 {
            exit(0);
        }
        if buffer == "exit\n" {
            exit(0)
        }
        match run(buffer.trim().to_string(), interpreter) {
            _ => continue,
        };
    }
}

fn run(source: String, interpreter: &mut Interpreter) -> Result<(), LangError> {
    let mut scanner = scanner::scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse()?;
    let mut resolver = Resolver::new(interpreter.clone());
    resolver.resolve_statements(statements.clone())?;
    if let Err(ref e) = resolver.interpreter.interpret(statements) {
        if let LangError::RuntimeError { message, line } = e {
            lang_error::error(*line, message.to_string())
        }
        return Err(e.clone());
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64)
    } else if args.len() == 2 {
        let path = Path::new(&args[1]);
        run_file(path, &mut interpreter)
    } else {
        run_prompt(&mut interpreter)
    }
}
