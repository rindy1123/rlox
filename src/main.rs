use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::exit;

mod lang_error;
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

fn run(source: String) -> String {
    println!("{}", source);
    source
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
