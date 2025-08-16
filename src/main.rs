use std::env;

use crate::mylib::{normal, repl};

mod list_logical_ops;
mod error_handling;
mod mylib;
mod quotation_ops;
mod string_ops;
mod stack;
mod combination_ops;
mod find_ops;

fn main() {
    // For REPL mode, run with
    // cargo run -- REPL

    // For normal mode, run with
    // cargo run

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let arg: &String = &args[1];

        if arg.eq_ignore_ascii_case("REPL") {
            println!("Running whole file...");
            repl();
        } else {
            println!("Unknown argument... starting interpreter");
            normal();
        }
    } else {
        println!("No argument given... starting interpreter");
        normal();
    }
}
