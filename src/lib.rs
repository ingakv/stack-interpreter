use crate::combination_ops::{combination_op, COMBINATION_OPS};
use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedVariable, IncompleteCodeBlock, IncompleteList, IncompleteString, ProgramFinishedWithMultipleValues, StackEmpty};
use crate::find_ops::handle_literal_and_operator;
use crate::find_ops::Operations::{Arithmetic, Block, List, Logical};
use crate::list_codeblock_ops::{CODEBLOCK_OPS, LIST_OPS};
use crate::logical_ops::{ARITHMETIC_OPS, LOGICAL_OPS};
use crate::stack::Type::{Block_, Bool_, Float_, Int_, List_, String_, Variable};
use crate::stack::{string_to_type, Stack, Type};
use crate::string_ops::{stack_string_io, IO_OPS, STACK_OPS, STRING_OPS};
use std::io;
use std::io::Write;

mod combination_ops;
mod error_handling;
mod find_ops;
mod list_codeblock_ops;
mod logical_ops;
mod stack;
mod string_ops;

pub fn t(input: &str) -> String {
    // Warning: don't move this function to another module, as integration tests in
    // directory `tests` with `cargo test` will only look into lib.rs, so make your parse and
    // execution functions public and import them here.

    // The following test function should:
    // 1. invoke parser (+lexer) with an input string
    // 2. invoke interpreter with tokens from parser as input
    // 3. transform the result to a string (tip: implement Display traits)

    let mut ans: Stack<Type> = read_stack(input.to_string(), Stack { elements: vec![] });

    ans = exec_stack(ans);

    print_stack_lib(ans)
}

fn print_stack_lib(mut ans: Stack<Type>) -> String {
    if let Some(elem) = ans.elements.pop() {
        elem.type_to_string()
    } else {
        String::new()
    }
}


pub fn run(normal: bool) {
    let mut stack: Stack<Type> = Stack::new();

    loop {

        if normal {
            print!("\n:q to quit\n:s to print the stack");
        }
        print!("\nbprog> ");
        io::stdout().flush().unwrap();

        // Reads user input
        let input = get_line();

        if normal && input == ":q" {

            // Prints the result of the operations
            if stack.is_empty() { print_error(StackEmpty); }
            else {

                // Execute the stack and print it out
                stack = exec_stack(stack);

                if stack.len() > 1 { print_error(ProgramFinishedWithMultipleValues) }
                if let Some(result) = stack.pop() { result.print(); }


                println!();
                stack.print_stack();
            }
            break;
        }

        else if normal && input == ":s" { stack.print_stack(); }

        else { stack = read_stack(input, stack.clone()); }

        if !normal {
            stack = exec_stack(stack);

            // Prints the stack
            stack.print_stack();
        }

    }
}

fn exec_stack(mut stack: Stack<Type>) -> Stack<Type> {
    let mut new_stack = Stack::new();

    // Loops through the stack as it was (stream pls💚) before execution 😵 (me rn since I am unable to can anymore)
    loop {

        let Some(elem) = stack.pop_front() else { break; };

        new_stack.push(elem.to_owned());

        // If there is a code block in the stack, execute it first
        new_stack = check_operator(elem.to_owned(), &mut new_stack.clone());

    }
    new_stack
}

fn read_stack(input: String, mut stack: Stack<Type>) -> Stack<Type> {

    // Splits up the different input variables
    let new_el: Vec<&str> = { input.split_whitespace().collect() };

    if new_el.is_empty() { print_error(StackEmpty); }

    // Variables to help join the elements together
    let str_buf: &mut Vec<&str> = &mut Vec::new();
    let mut is_str: bool = false;

    let bl_buf: &mut Vec<Type> = &mut Vec::new();
    let mut is_block: bool = false;

    let li_buf: &mut Vec<Type> = &mut Vec::new();
    let mut is_list: bool = false;

    let sub_buf: &mut Vec<Type> = &mut Vec::new();
    let mut is_sublist = false;

    for i in new_el {

        // Remove extra characters from the element
        let elem =  i.trim().trim_matches(|c| c == ' ' || c == '"');

        // Does the element start or end with a quote?
        let has_start_quote = i.trim().starts_with('"');
        let has_end_quote = i.trim().ends_with('"');

        // Does the element start with a [ or end with a ]?
        let is_list_start = i.trim().starts_with('[');
        let is_list_end = i.trim().ends_with(']');


        //////////////// String /////////////////

        // If it is the end of the string, 
        // or if the string contains a single word, 
        // and it is not a single quotation mark
        if ((has_start_quote || has_end_quote) && is_str)
            || (has_start_quote && has_end_quote)
            && i.trim() != "\"" {


            // Remove the last whitespace
            if !str_buf.is_empty() &&
                str_buf.last().unwrap().trim().is_empty() &&
                has_start_quote && has_end_quote {
                str_buf.pop();
            }

            if !elem.is_empty() { str_buf.push(elem); }

            // If we are in a list, copy the new list over
            if is_list { li_buf.push(String_(str_buf.concat())); }

            else if is_block { bl_buf.push(String_(str_buf.concat())); }

            // Join the vector together to form a sentence / string and send it to the stack
            else { stack.push(String_(str_buf.concat().to_string())) }

            // Reset the buffer so that a potential new string can be read
            str_buf.clear();

            is_str = false;

        }

        // If a string is currently being read, push it to the buffer, with a whitespace after
        else if has_start_quote || is_str {

            // Push the element to the buffer
            if !elem.is_empty() {
                str_buf.push(elem);

                // Add a whitespace between elements / words
                str_buf.push(" ");
            }
            is_str = true;
        }


        //////////////// List /////////////////

        // If it is the end of the list, 
        // or if the list contains a single element
        else if (is_list && is_list_end) ||
            (is_list_start && is_list_end) {

            // If the list is a sublist, continue reading it
            if is_sublist {
                push_to_vec(elem, sub_buf);
                li_buf.push(List_(sub_buf.to_owned()));
                sub_buf.clear();
                is_sublist = false;
            }

            else {
                // If the list is not a sublist, set is_list to false
                is_list = false;

                // Join the vector together to form a list and send it to the stack
                stack.push(List_(li_buf.to_owned()));

                // Reset the buffer so that a potential new list can be read
                li_buf.clear();
            }
        }

        // If it is the start of a list
        else if is_list_start {
            // If it is a sublist, push the sublist one to the list buffer
            if is_list {
                push_to_vec(elem, sub_buf);
                is_sublist = true;
            }
            else {
                // If it is not a sublist, push the element to the regular list
                push_to_vec(elem, li_buf);
                is_list = true;
            }
        }




        //////////////// Code block AKA quotation /////////////////

        // If it is the start of a block
        else if i.contains('{') { is_block = true; }


        // If it is the end of the block
        else if i.contains('}') {

            // If we are in a list, copy the new list over
            if is_list { li_buf.push(Block_(bl_buf.to_owned())); }

            // Push the code block to the stack or execute it
            else { stack.push(Block_(bl_buf.to_owned())); }

            // Reset the buffer so that a potential new block can be read
            bl_buf.clear();
            is_block = false;
        }



        //////////////// Push to buffer /////////////////

        // If a block is currently being read, push it to the buffer
        else if is_block { push_to_vec(elem, bl_buf); }

        // If a sublist is currently being read, push it to the buffer, with a comma after
        else if is_sublist { push_to_vec(elem, sub_buf); }

        // If it is not a sublist, push the element to the regular list
        else if is_list { push_to_vec(elem, li_buf); }



        //////////////// Push to stack /////////////////

        else {
            match string_to_type(elem) {
                Int_(elem) => stack.push(Int_(elem)),
                Float_(elem) => stack.push(Float_(elem)),
                Bool_(elem) => stack.push(Bool_(elem)),
                String_(elem) => stack.push(String_(elem)),
                Variable(elem) => stack.push(Variable(elem)),
                _ => {}
            };
        }
    }

    // Error handling for incomplete code blocks, lists, and strings
    if is_str { print_error(IncompleteString); }
    if is_block { print_error(IncompleteCodeBlock); }
    if is_list || is_sublist { print_error(IncompleteList); }

    stack
}

pub(crate) fn check_operator(c: Type, stack: &mut Stack<Type>) -> Stack<Type> {

    let c_string = c.type_to_string_trimmed().to_lowercase();
    let op = c_string.as_str();

    let new = &mut stack.clone();

    // Remove the operator
    new.pop();

    let new_stack =

        if COMBINATION_OPS.contains(&op) { combination_op(stack) }

        else if c.is_block() { handle_literal_and_operator(Block, stack) }

        else if ARITHMETIC_OPS.contains(&op) { handle_literal_and_operator(Arithmetic, stack) }

        else if LOGICAL_OPS.contains(&op) { handle_literal_and_operator(Logical, stack) }

        else if LIST_OPS.contains(&op) { handle_literal_and_operator(List, stack) }

        else if IO_OPS.contains(&op) ||
            STRING_OPS.contains(&op) ||
            STACK_OPS.contains(&op) { stack_string_io(op, new) }


        else { stack.to_owned() };

    new_stack
}

// Pattern matches the types to push to vector
fn push_to_vec(i: &str, vec: &mut Vec<Type>) {
    let elem =  i.trim().trim_matches(|c| c == ' ' || c == '"' || c == '[' || c == ']');
    if !elem.is_empty() {
        match string_to_type(elem) {
            Int_(elem) => vec.push(Int_(elem)),
            Float_(elem) => vec.push(Float_(elem)),
            Bool_(elem) => vec.push(Bool_(elem)),
            String_(elem) => vec.push(String_(elem)),
            Variable(elem) => vec.push(Variable(elem)),
            _ => { print_error(ExpectedVariable) }
        };
    }
}


pub(crate) fn get_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim_end().to_string()
}


pub(crate) fn is_op(el: &str) -> bool {
    CODEBLOCK_OPS.contains(&el) ||
    IO_OPS.contains(&el) ||
    STACK_OPS.contains(&el) ||
    STRING_OPS.contains(&el) ||
    ARITHMETIC_OPS.contains(&el) ||
    LOGICAL_OPS.contains(&el) ||
    LIST_OPS.contains(&el) ||
    COMBINATION_OPS.contains(&el)
}

