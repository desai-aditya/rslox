use std::io::{self, stdin, Write};
use std::{env::args, fs::read_to_string, process::exit};

use scanlex::{Scanner, Token};

#[derive(Debug)]
pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }
    pub fn run(&self, source: String) -> io::Result<()> {
        let mut scan = Scanner::new(&source);
        loop {
            match scan.get() {
                Token::End => break,
                token => println!("{:?}", token),
            }
        }
        Ok(())
    }

    pub fn run_file(self, filename: String) -> io::Result<()> {
        let file_string = read_to_string(filename)?;
        self.run(file_string)
    }

    pub fn run_prompt(self) -> io::Result<()> {
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
        }
        Ok(())
    }
}
