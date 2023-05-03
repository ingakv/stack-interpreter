use std::env;
use crate::mylib::{normal, repl};
use crate::structs::{Stack, Type};

mod arithmetic_ops;
mod list_ops;
mod logical_ops;
mod mylib;
mod string_ops;
mod error_handling;
mod structs;

fn main() {

    // For REPL mode, run with
    // cargo run -- REPL

    // For normal mode, run with
    // cargo run

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        let arg: &String = &args[1];

        if arg == "REPL" {
            println!("Running whole file...");
            repl();
        }

        else {
            println!("Unknown argument... starting interpreter");
            normal();
        }
    }

    else {
        println!("No argument given... starting interpreter");
        normal();
    }

}
