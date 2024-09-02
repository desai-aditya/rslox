use std::io::{self, stdin, Write};
use std::{env::args, fs::read_to_string, process::exit};

use scanlex::{Scanner, Token};

fn run(source: String) -> io::Result<()> {
    let mut scan = Scanner::new(&source);
    loop {
        match scan.get() {
            Token::End => break,
            token => println!("{:?}", token),
        }
    }
    Ok(())
}

fn run_file(filename: String) -> io::Result<()> {
    let file_string = read_to_string(filename)?;
    run(file_string)
}

fn run_prompt() -> io::Result<()> {
    loop {
        let mut buf: String = "".to_owned();
        print!("> ");
        io::stdout().flush()?;
        if let Err(e) = stdin().read_line(&mut buf) {
            eprintln!("{:}", e);
        }
        if buf.is_empty() {
            break;
        }
        run(buf)?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = match args().len() {
        1 => run_prompt(),
        2 => run_file(args().next().to_owned().expect("file should run fine")),
        _ => {
            eprintln!("Usage: rslox [script]");
            exit(64);
        }
    } {
        eprintln!("{:?}", e);
        exit(64);
    };
}
