use std::env;
use crate::mylib::run_program;

mod arithmetic_ops;
mod list_ops;
mod logical_ops;
pub mod mylib;
mod string_ops;

//mod test;

fn main() {

    // Run with
    // cargo run -- REPL

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        let arg: &String = &args[1];

        if arg == "REPL" {
            println!("Running whole file...");
            run_program(true);
        }

        else {
            println!("Unknown argument... starting interpreter");
            run_program(false);
        }
    }

    else {
        println!("No argument given... starting interpreter");
        run_program(false);
    }

}
