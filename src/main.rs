use std::env;
use bprog::run;

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
            run(false);
        } else {
            println!("Unknown argument... starting interpreter");
            // REPL mode (looping through and executing the code for each user input)
            run(true);
        }
    } else {
        println!("No argument given... starting interpreter");
        run(true);
    }
}
