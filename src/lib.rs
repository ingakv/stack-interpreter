
mod arithmetic_ops;
mod list_ops;
mod logical_ops;
mod mylib;
mod string_ops;
mod error_handling;
mod structs;
mod quotation_ops;

use crate::mylib::{program_loop};
use crate::structs::{Stack, Type};


pub fn t(input: &str) -> String {
    // Warning: don't move this function to another module, as integration tests in
    // directory `tests` with `cargo test` will only look into lib.rs, so make your parse and
    // execution functions public and import them here.

    // The following test function should:
    // 1. invoke parser (+lexer) with input string
    // 2. invoke interpreter with tokens from parser as input
    // 3. transform the result to a string (tip: implement Display traits)

    let ans: Stack<Type> = program_loop(input.to_string(), Stack{ elements: vec![] }, true);

    print_stack_lib(ans)

}


pub fn print_stack_lib(mut ans: Stack<Type>) -> String {
    if let Some(elem) = ans.elements.pop() { elem.type_to_string() }
    else { "".to_string() }
}

