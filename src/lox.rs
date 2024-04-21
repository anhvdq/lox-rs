mod ast;
mod parser;
mod scanner;
mod interpreter;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

use ast::tree::AstPrinter;
use ast::tree::AstVisitor;
use parser::Parser;
use scanner::scanner::Scanner;

fn main() {
    // Get list of arguments
    let args: Vec<String> = env::args().collect();
    // The first arg is always the app name
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        // Arg is a path
        run_file(&args[1]);
    } else {
        // No arg => Run interactive mod
        run_prompt();
    }
}

fn run_file(path: &String) {
    let source = fs::read_to_string(path).expect("File should be able to read");
    if run(&source).is_err() {
        process::exit(65);
    };
}

fn run_prompt() {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!("> ");
        stdout.flush().unwrap();
        stdin.read_line(&mut buf).expect("Failed to read line");
        let line = buf.to_string();
        buf.clear();

        if run(&line).is_err() {
            process::exit(65);
        };
    }
}

fn run(source: &String) -> Result<String, String> {
    if source.contains("err") {
        return Err("Error".to_string());
    }

    let scanner = Scanner {
        source: source.to_owned(),
    };

    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser: Parser = Parser::new(tokens);
            if let Ok(expr) = parser.parse() {
                let mut visitor = AstPrinter;
                let val = visitor.process(&expr, None);
                print!("\n{}\n", val)
            };
        }
        Err(scan_err) => error(scan_err.line, scan_err.message),
    };

    Ok("Success".to_string())
}

fn error(line: usize, message: String) {
    report(line, &String::from(""), message);
}

fn report(line: usize, where_err: &String, message: String) {
    println!("[line {}] Error{}: {}", line, where_err, message)
}
