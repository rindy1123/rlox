use std::env;
use std::fs;
use std::process::{exit, Command};

mod expr;
mod stmt;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: generate_ast <output directory>");
        exit(64)
    }
    let output_dir = if args.len() == 1 { "src" } else { &args[1] };
    fs::create_dir_all(output_dir).unwrap();
    expr::define_ast(output_dir);
    stmt::define_ast(output_dir);
    Command::new("cargo").args(&["fmt"]).output().unwrap();
    println!("Generated");
}
