mod arithmetic_ops;
mod list_ops;
mod logical_ops;
mod mylib;
mod string_ops;


use crate::mylib::program_loop;


pub fn t(input: &str) -> String {
    // Warning: don't move this function to another module, as integration tests in
    // directory `tests` with `cargo test` will only look into lib.rs, so make your parse and
    // execution functions public and import them here.

    // The following test function should:
    // 1. invoke parser (+lexer) with input string
    // 2. invoke interpreter with tokens from parser as input
    // 3. transform the result to a string (tip: implement Display traits)

    let mut stack: Vec<String> = Vec::new();

    stack = program_loop(input.to_string(), stack.clone(), true);

//    add_stack(&mut list, input.to_string());

//    exec_stack(&mut list);

    stack.pop().unwrap()

}