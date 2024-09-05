pub mod scanner;
pub mod token;

use std::io::{self, stdin, Write};
use std::{fs::read_to_string, process::exit};

//use scanlex::{Scanner, Token};
use crate::lox::scanner::Scanner;
use crate::lox::token::Token;

#[derive(Debug)]
pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, "".to_string(), message);
    }

    pub fn report(&mut self, line: usize, location: String, message: String) {
        eprintln!("[line {}]: Error {}: {}", line, location, message);
        self.had_error = true;
    }

    pub fn run(&mut self, source: String) -> io::Result<()> {
        let mut scan = Scanner::new(source);
        match scan.scan_tokens() {
            Ok(token_vec) => println!("{:?}", token_vec),
            Err((line, e)) => self.error(line, e),
        }
        Ok(())
    }

    pub fn run_file(mut self, filename: String) -> io::Result<()> {
        let file_string = read_to_string(filename)?;
        self.run(file_string)?;
        if self.had_error {
            exit(-1);
        }
        Ok(())
    }

    pub fn run_prompt(mut self) -> io::Result<()> {
        loop {
            let mut buf: String = "".to_owned();
            print!("> ");
            io::stdout().flush()?;
            if let Err(e) = stdin().read_line(&mut buf) {
                eprintln!("{:}", e);
                break;
            }
            if buf.is_empty() {
                break;
            }
            self.run(buf)?;
            self.had_error = false;
        }
        Ok(())
    }
}
