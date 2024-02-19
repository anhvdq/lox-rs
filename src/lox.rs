use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

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
    run(&source).expect("msg");
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
            break;
        };
    }
}

fn run(source: &String) -> Result<String, String> {
    Ok("Success".to_string())
}