mod types;
mod parser;
mod executor;


use std::io::{self, Write};
use types::*;
use parser::parse;
use crate::types::Database;
use crate::executor::execute;

fn main() {
    println!("Rusted engine v0.1");
    println!("Type EXIT to quit.");
    let mut db = Database::new();

    loop {
        print!("db> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match parse(input) {
            Ok(Command::Exit) => {
                println!("Bye!");
                break;
            }
            Ok(cmd) => match execute(&mut db, cmd) {
                Ok(msg) => println!("{}", msg),
                Err(e) => println!("Error: {}", e),
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
