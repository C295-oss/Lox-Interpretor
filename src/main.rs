use std::env;
use std::fs;
use std::io::{self, Write}; // Import io for stdin/stdout and Write trait
use std::process;

use crate::lexer::Scanner;
pub mod lexer;


struct Lox {
    pub had_error_: bool, // This needs to be mutable
}

impl Lox {
    pub fn new() -> Self {
        Lox {
            had_error_: false,
        }
    }

    // This method needs `&mut self` because it will call methods that modify `had_error_`
    pub fn run_lox_app(&mut self, args: Vec<String>) {
        if args.len() > 1 {
            eprintln!("Usage: rlox [script]");
            process::exit(64);
        } else if args.len() == 1 {
            self.run_file(&args[0]); 
        } else {
            self.run_prompt();
        }
    }

    // ////////////////////////////////////////////////////////////////
    // Dealing with Input

    pub fn run_file(&mut self, file_path: &str) {
        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", file_path, e);
                process::exit(74); // Exit with a file-related error code
            }
        };
        self.run(contents);
        if self.had_error_ {
            process::exit(65); // Exit with a runtime error code
        }
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();

        loop {
            // If an error occurred in the *previous* line, exit.
            // For a prompt, you might want to reset the error for each new line.
            if self.had_error_ { // Corrected field name
                process::exit(65);
            }

            print!("> ");
            io::stdout().flush().unwrap(); // Ensure the prompt is displayed

            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) => break, // EOF (Ctrl+D on Unix, Ctrl+Z on Windows)
                Ok(_) => {
                    let trimmed_line = line.trim().to_string(); // Trim whitespace
                    if trimmed_line.is_empty() {
                        // If line is empty after trimming, continue to next prompt
                        // This handles just pressing Enter without input
                        self.had_error_ = false; // Reset error flag for the next prompt iteration
                        continue;
                    }
                    self.run(trimmed_line);
                    self.had_error_ = false; // Reset error for the next line in the prompt
                },
                Err(error) => {
                    eprintln!("Error reading line: {}", error);
                    break; // Exit on severe input error
                }
            }
        }
    }

    // ////////////////////////////////////////////////////////////////
    // Error handling

    fn error(&mut self, line: i32, msg: String) {
        self.report(line, "".to_string(), msg);
    }

    fn report(&mut self, line: i32, loc: String, msg: String) {
        self.had_error_ = true;
        eprintln!("[line {}] Error {}: {}", line, loc, msg);
    }

    fn run(&mut self, source_code: String) { // Renamed `file_path` to `source_code` for clarity
        let mut scanner = Scanner::new(source_code);
        let tokens = scanner.scan_tokens(); // Renamed scanTokens to scan_tokens (Rust convention)

        for t in tokens {
            println!("{}", t);
        }
    }
}


fn main() {
    let mut lox = Lox::new();
    let args: Vec<String> = env::args().skip(1).collect(); // Collect command-line arguments
    lox.run_lox_app(args);
}