use rslox::lox::Lox;
use std::env::args;
use std::process::exit;

fn main() {
    let lox = Lox::new();
    if let Err(e) = match args().len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(args().next().to_owned().expect("file should run fine")),
        _ => {
            eprintln!("Usage: rslox [script]");
            exit(64);
        }
    } {
        eprintln!("{:?}", e);
        exit(64);
    };
}
