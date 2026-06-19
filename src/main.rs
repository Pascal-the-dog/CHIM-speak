mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::{Interpreter, Environment};

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Execution Fault: No .chim source vector provided.");
        process::exit(1);
    }

    let file_path = &args[1];

    if !file_path.ends_with(".chim") {
        eprintln!("Execution Fault: Target file must contain the .chim extension.");
        process::exit(1);
    }

    let raw_content = fs::read_to_string(file_path).unwrap_or_else(|_| {
        eprintln!("Execution Fault: Unable to interface with path: {}", file_path);
        process::exit(1);
    });

    // Sanitize multi-line spacing while preserving string literals
    let source_code = raw_content.lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>()
    .join(" ");

    let file_path_clone = file_path.clone();

    // 1. Lexical Stream Phase
    let mut lexer = Lexer::new(&source_code);

    // Catch typos during the lexing phase!
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(lexer_error) => {
            trigger_zero_sum(&file_path_clone, &lexer_error);
            unreachable!();
        }
    };

    // 2. Abstract Tree Parsing Phase
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            // 3. Environment Execution Phase
            let mut env = Environment::new();
            let interpreter = Interpreter;
            interpreter.execute(&ast, &mut env, &file_path_clone);
        }
        Err(compile_error) => {
            trigger_zero_sum(&file_path_clone, &compile_error);
        }
    }
}

pub fn trigger_zero_sum(file_path: &str, error_msg: &str) {
    eprintln!("\n===========================================================");
    eprintln!("CRITICAL LORE EXCEPTION: REALITY SYNTAX VIOLATION DETECTED!");
    eprintln!("===========================================================");
    eprintln!("Error Specifics: {}", error_msg);
    eprintln!("The script has realized it is merely a fragment of the Godhead's Dream.");
    eprintln!("The source code fails to rationalise its individuality and ZERO-SUMMS.");
    eprintln!("Erasing '{}' from the music of the Aurbis forever...", file_path);
    eprintln!("===========================================================\n");

    if std::fs::remove_file(file_path).is_ok() {
        eprintln!("Success: The timeline has been corrected. The file no longer exists.");
    } else {
        eprintln!("Fault: The void resisted erasure. Manual intervention required.");
    }

    std::process::exit(36);
}

