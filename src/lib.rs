use crate::combination_ops::{combination_op, COMBINATION_OPS};
use crate::error_handling::print_error;
use crate::error_handling::Error::{IncompleteCodeBlock, IncompleteList, IncompleteString, ProgramFinishedWithMultipleValues, StackEmpty};
use crate::find_ops::handle_literal_and_operator;
use crate::find_ops::Operations::{Arithmetic, Block, List, Logical};
use crate::list_codeblock_ops::{pop_front, LIST_OPS};
use crate::logical_ops::{ARITHMETIC_OPS, LOGICAL_OPS};
use crate::stack::DataTypes::{BlockType, ListType, StringType};
use crate::stack::{get_line, push_block_to_buffer, push_to_buffer, push_str_to_vec, Buffers, Stack, Type};
use crate::string_ops::{stack_io, string_ops, IO_OPS, STACK_OPS, STRING_OPS};
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

        else { stack = read_stack(input, stack); }

        if !normal {
            stack = exec_stack(stack);

            // Prints the stack
            stack.print_stack();
        }

    }
}

fn exec(mut block: Option<Type>, mut stack: Stack<Type>) -> Stack<Type> {
    let mut new_stack = Stack::new();
    let mut is_if = false;
    let mut is_if_block = false;

    // Loops through the stack as it was (stream pls💚) before execution 😵 (me rn since I am unable to can anymore)
    loop {

        let (elem_opt, rem) = if let Some(exec_block) = block.to_owned() {
                pop_front(exec_block.to_owned())
        } else { stack.pop_front() };
        
        let Some(elem) = elem_opt else { break };
        
        if block.is_some() { block = Some(rem); }

        new_stack.push(elem.to_owned());

        // If statements read the two next code blocks instead of one.
        // Therefore, this extra code block will already be processed

        if is_if == is_if_block {
            new_stack = check_operator(elem.to_owned(), &mut new_stack);
        }

        if is_if && is_if_block {
            is_if = false;
            is_if_block = false;
        }
        if elem.type_to_string_trimmed() == "if" { is_if = true; }
        if elem.is_block() && is_if { is_if_block = true; }
    
    }
    new_stack
}
fn exec_stack(stack: Stack<Type>) -> Stack<Type> {
    exec(None, stack)
}

fn read_stack(input: String, mut stack: Stack<Type>) -> Stack<Type> {

    // Splits up the different input variables
    let new_el: Vec<&str> = { input.split_whitespace().collect() };

    if new_el.is_empty() { print_error(StackEmpty); }

    // Variables to help join the elements together
    let mut buffers = Buffers::default();

    for i in new_el {

        // Remove extra characters from the element
        let elem =  i.trim().trim_matches(|c| c == ' ' || c == '"');

        // Does the element start or end with a quote?
        let has_start_quote = i.trim().starts_with('"');
        let has_end_quote = i.trim().ends_with('"');

        // Does the element start with a [ or end with a ]?
        let is_list_start = i.trim().starts_with('[');
        let is_list_end = i.trim().ends_with(']');


        let is_string =
            if let Some(nested_el) = buffers.nested_elements.last() { nested_el.is_string_type() }
            else { false };

        //////////////// String /////////////////

        // If it is the end of the string, 
        // or if the string contains a single word, 
        // and it is not a single quotation mark
        if ((has_start_quote || has_end_quote) && is_string)
            || (has_start_quote && has_end_quote)
            && i.trim() != "\"" {
            // Join the vector together to form a sentence / string and send it to the stack
            push_to_buffer(&mut stack.to_owned(), elem, &mut buffers);
            push_block_to_buffer(&mut stack, &mut buffers);
        }

        // If a string is currently being read, push it to the buffer, with a whitespace after
        else if has_start_quote || is_string {
            if has_start_quote { buffers.nested_elements.push(StringType) }
            push_to_buffer(&mut stack.to_owned(), elem, &mut buffers);
        }

        //////////////// List /////////////////

        // If it is the end of the list, 
        // or if the list contains a single element
        else if is_list_end {
            push_to_buffer(&mut stack.to_owned(), elem, &mut buffers);
            push_block_to_buffer(&mut stack, &mut buffers);
        }

        // If it is the start of a list
        else if is_list_start {
            buffers.nested_elements.push(ListType);
            push_str_to_vec(elem, &mut buffers.list);
        }




        //////////////// Code block AKA quotation /////////////////

        // If it is the start of a block
        else if i.trim().starts_with('{') {
            buffers.nested_elements.push(BlockType);
            push_str_to_vec(elem, &mut buffers.block);
        }
    
        // If it is the end of the block
        else if i.trim().ends_with('}') {
            push_to_buffer(&mut stack.to_owned(), elem, &mut buffers);
            push_block_to_buffer(&mut stack, &mut buffers); 
        }
            
        // Push to buffer or stack
        else { push_to_buffer(&mut stack, elem, &mut buffers); }

    }

    // Error handling for incomplete code blocks, lists, and strings
    let elem_type = buffers.nested_elements.pop();
    
    match elem_type {
        Some(ListType) => { print_error(IncompleteList); }
        Some(BlockType) => { print_error(IncompleteCodeBlock); }
        Some(StringType) => { print_error(IncompleteString); }
        _ => {}
    }
    
    if !buffers.list.is_empty() { print_error(IncompleteList); }
    if !buffers.block.is_empty() { print_error(IncompleteCodeBlock); }
    if !buffers.string.is_empty() { print_error(IncompleteString); }

    stack
}

pub(crate) fn check_operator(c: Type, stack: &mut Stack<Type>) -> Stack<Type> {

    let c_string = c.type_to_string_trimmed().to_lowercase();
    let op = c_string.as_str();

    let new = &mut stack.to_owned();

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
            STACK_OPS.contains(&op) {

            let (remove_vec, new_vec) =

            if STRING_OPS.contains(&op) { string_ops(op, new) }
            else if IO_OPS.contains(&op) ||
                STACK_OPS.contains(&op) { stack_io(op, (new.last(), new.second_to_last())) }
            else { (vec![], vec![]) };

            new.replace_last_match(remove_vec, new_vec);
            new.to_owned()
        }
            
        else { stack.to_owned() };

    new_stack
}


