use std::io;
use std::io::Write;
use std::io::stdout;
use std::env;

pub mod lexer;

fn main() {
    // let mut subcommand = String::new();

    rlpl();

    let mut args = env::args();
    let subcommand = args.nth(1).expect("Missing subcommand!");


    // io::stdin().read_line(&mut subcommand).expect("Failed to read from stdin");

    match subcommand.as_str() {
        "rlpl" => rlpl(),
        "rppl" => rppl(),
        _ => println!("Unknown subcommand provided {}", subcommand),
    }
}

fn rlpl() {
    let mut code = String::new();

    loop {
        print!("> ");
        stdout().flush().expect("Failed to write to stdout");
        io::stdin().read_line(&mut code).expect("rlpl: Failed to read from stdin");
        let tokens = lexer::tokenize(code.as_str());
        print!("{:?}", tokens);
    }
}

fn rppl() {
    let mut code = String::new();

    loop {
        print!("> ");
        stdout().flush().expect("Failed to write to stdout");
        io::stdin().read_line(&mut code).expect("rppl: Failed to read from stdin");
        match code {
            _ => println!("rppl: Failed to parse code from stdin")
        }
    }
}
