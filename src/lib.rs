
mod arithmetic_ops;
mod list_ops;
mod logical_ops;
mod mylib;
mod string_ops;
mod error_handling;
mod structs;
mod quotation_ops;

use crate::mylib::{is_op, program_loop};
use crate::structs::{Stack, Type};
use crate::structs::Type::{Block_, Bool_, Float_, Int_, List_, String_};


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
    if let Some(elem) = ans.elements.pop() { stack_to_string(elem) }
    else { "".to_string() }
}



fn stack_to_string(ans: Type) -> String {

    match ans {
        Int_(str) => {str.to_string()}
        Float_(str) => {
            if !str.to_string().contains('.') { format!("{}.0", str.to_string())}
            else { str.to_string() }
        }
        Bool_(str) => {
            if str.to_string() == "true" { "True".to_string()}
            else { "False".to_string() }
        }
        String_(str) => {
            if !is_op(str.as_str()) { ("\"".to_owned() + &str + "\"").to_string() }
            else { str }
        }
        List_(str) => {

            let mut new_li: Vec<String> = vec![];

            new_li.push("[".to_string());

            if !str.is_empty() {
                for i in str {
                    new_li.push(stack_to_string(i));
                    new_li.push(",".to_string());
                }
                new_li.pop();
            }
            new_li.push("]".to_string());

            new_li.concat()

        }

        Block_(str) => {

            let mut new_li: Vec<String> = vec![];

            new_li.push("{ ".to_string());

            if !str.is_empty() {
                for i in str {
                    new_li.push(stack_to_string(i));
                    new_li.push(" ".to_string());
                }
                new_li.pop();
            }
            new_li.push(" }".to_string());

            new_li.concat()

        }
    }

}