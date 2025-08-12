use std::io;
use std::io::Write;
use std::ops::Neg;
use crate::arithmetic_ops::{find_arithmetic, ARITHMETIC_OPS};
use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedNumber, ExpectedVariable, IncompleteList, IncompleteQuotation, IncompleteString, ProgramFinishedWithMultipleValues, StackEmpty};
use crate::list_ops::{find_list, list_op, LIST_OPS};
use crate::logical_ops::{find_logical, LOGICAL_OPS};
use crate::quotation_ops::{quotation, QUOTATION_OPS};
use crate::stack::Type::{Block_, Bool_, Float_, Int_, List_, String_};
use crate::stack::{string_to_type, is_block, Stack, Type};
use crate::string_stack_io_ops::{parse_string, simple_io, stack_op, IO_OPS, STACK_OPS, STRING_OPS};

#[allow(dead_code)]
pub fn normal() {
    let mut stack: Stack<Type> = Stack::new();

    loop {

        print!("\n:q to quit\n:s to print the stack\nbprog> ");
        io::stdout().flush().unwrap();

        // Reads user input

        let input = get_line();

        if input == ":q" {

            // Prints the result of the operations
            if stack.is_empty() { print_error(StackEmpty); }
            else {

                // Execute the stack and print it out
                stack = exec_stack(stack.to_owned());

                if stack.len() > 1 {print_error(ProgramFinishedWithMultipleValues)}
                if let Some(result) = stack.pop() { result.print(); }


                println!();
                stack.print_stack();

            }
            break;
        }

        // Prints the stack
        else if input == ":s" { stack.print_stack(); }

        else { stack = read_stack(input, stack.clone()); }

    }
}




// REPL mode (looping through and executing the code for each user input)
#[allow(dead_code)]
pub fn repl() {
    let mut stack: Stack<Type> = Stack::new();

    loop {
        print!("\nbprog> ");
        io::stdout().flush().unwrap();

        // Reads user input
        let input = get_line();

        let new = read_stack(input, stack.clone());

        stack = exec_stack(new);

        // Prints the stack
        stack.print_stack();

    }
}



pub fn exec_stack(mut stack: Stack<Type>) -> Stack<Type> {
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

pub fn read_stack(input: String, mut stack: Stack<Type>) -> Stack<Type> {

    // Splits up the different input variables
    let new_el: Vec<&str> = { input.split_whitespace().collect() };

    if new_el.is_empty() { print_error(StackEmpty); }

    // Variables to help join the elements together
    let mut str_buf: Vec<&str> = vec![];
    let mut is_str: bool = false;

    let mut bl_buf: Vec<Type> = vec![];
    let mut is_block: bool = false;

    let mut li_buf: Vec<Type> = vec![];
    let mut is_list: bool = false;

    let mut sub_buf: Vec<Type> = vec![];
    let mut is_sublist = false;
    
    for i in new_el {

        //////////////// String /////////////////

        // Remove quotes or whitespace
        let elem =  i.trim().trim_matches(|c| c == ' ' || c == '"');
        
        // Does the element start or end with a quote?
        let has_start_quote = i.trim().starts_with('"');
        let has_end_quote = i.trim().ends_with('"');
        
        // If it is the end of the string, 
        // or if the string contains a single word, 
        // and it is not a single quotation mark
        if ((has_start_quote || has_end_quote) && is_str)
            || (has_start_quote && has_end_quote)
            && i.trim() != "\"" {


            // Remove the last whitespace
            if !str_buf.is_empty() &&
                str_buf.last().unwrap().trim().is_empty() {
                str_buf.pop();
            }
            
            if !elem.is_empty() { str_buf.push(elem); }

            // If we are in a list, copy the new list over
            if is_list {
                li_buf.push(String_(str_buf.concat()));
            }

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

        // If it is the start or the end of a list
        else if i.contains('[') {

            // If it is a sublist, push the sublist one to the list buffer
            if is_list {
                is_sublist = true;
            }
            is_list = true;
        }


        // If it is the end of the list
        else if i.contains(']') {

            // If the list is not a sublist, set is_list to false
            // If the list is a sublist, continue reading it
            if !is_sublist {

                // Join the vector together to form a list and send it to the stack
                stack.push(List_(li_buf.to_owned()));

                is_list = false;

                // Reset the buffer so that a potential new list can be read
                li_buf.clear();

            }

            else {
                li_buf.push(List_(sub_buf.to_owned()));
                sub_buf.clear();
                is_sublist = false;
            }
        }




        //////////////// Code block AKA quotation /////////////////


        // If it is the start of a block
        else if i.contains('{') { is_block = true; }


        // If it is the end of the block
        else if i.contains('}') {

            // If we are in a list, copy the new list over
            if is_list {
                li_buf.push(Block_(bl_buf.to_owned()));
            }

            // Push the code block to the stack or execute it
            else { stack.push(Block_(bl_buf.to_owned())); }


            is_block = false;

            // Reset the buffer so that a potential new block can be read
            bl_buf.clear();

        }



        //////////////// Push to buffer /////////////////

        // If a block is currently being read, push it to the buffer
        else if is_block { bl_buf = push_to_vec(i, bl_buf.to_owned()); }

        // If a sublist is currently being read, push it to the buffer, with a comma after
        else if is_sublist { sub_buf = push_to_vec(i, sub_buf.to_owned()); }

        // If it is not a sublist, push the element to the regular list
        else if is_list { li_buf = push_to_vec(i, li_buf.to_owned()); }



        //////////////// Push to stack /////////////////

        else {
            match string_to_type(i) {
                Int_(i) => stack.push(Int_(i.to_owned())),
                Float_(i) => stack.push(Float_(i.to_owned())),
                Bool_(i) => stack.push(Bool_(i.to_owned())),
                String_(i) => stack.push(String_(i.to_owned())),
                _ => {}
            };
        }
    }
    
    
    if is_str {
        print_error(IncompleteString);
    }
    if is_block {
        print_error(IncompleteQuotation);
    }
    if is_list || is_sublist {
        print_error(IncompleteList);
    }

    stack.to_owned()

}




pub(crate) fn check_operator(c: Type, stack: &mut Stack<Type>) -> Stack<Type> {
    
    let old_stack = stack.clone();
    
    // If there is code, execute it first
    if stack.has_code() {

/*
        // Find and extract the code, list, and quotation operator into a separate stack
        let block_stack = find_block(&mut stack.clone());

        // Push the result of the code block to the front of the regular stack
        stack.reverse();

        for el in do_quotation(block_stack.to_owned()).elements {
            stack.push(el);
        }

        stack.reverse();

        stack.replace_last_match(block_stack.elements, String_(String::new()));
        */

        stack.to_owned()

    }


    else {

        // Remove the operator
        stack.replace_last_match(vec![c.clone()], String_(String::new()));

        let c_string = c.type_to_string();
        let op = c_string.as_str();

        let new_stack =

            // Ignores ""
            if c.is_empty() { stack.clone() }

            else if c == String_("length".to_string()) { 
                length(stack)
            }

            else if c == String_("==".to_string()) {
                compare(stack)
            }

            else if c == String_("not".to_string()) { 
                invert(stack)
            }

            else if ARITHMETIC_OPS.contains(&op) {
                stack.push(c);
                let mut new = &mut stack.clone();
                let mut new2 = new.clone();

                find_arithmetic(&mut new, &mut new2, false)
            }

            else if LOGICAL_OPS.contains(&op) {
                stack.push(c);
                let mut new = &mut stack.clone();
                let mut new2 = new.clone();

                find_logical(&mut new, &mut new2, false)
            }

            else if STRING_OPS.contains(&op) { parse_string(op, stack) }

            else if LIST_OPS.contains(&op) {
                stack.push(c);
                let mut new = &mut stack.clone();
                let mut new2 = new.clone();

                find_list(&mut new, &mut new2, false)
            }

            else if STACK_OPS.contains(&op) { stack_op(op, stack) }

            else if IO_OPS.contains(&op) { simple_io(op, stack) }

            else { old_stack.to_owned() };

        new_stack
    }
}



// Pattern matches the types to push to vector
pub(crate) fn push_to_vec(i: &str, mut stack: Vec<Type>) -> Vec<Type> {
    match string_to_type(i) {
        Int_(i) => stack.push(Int_(i.to_owned())),
        Float_(i) => stack.push(Float_(i.to_owned())),
        Bool_(i) => stack.push(Bool_(i.to_owned())),
        String_(i) => stack.push(String_(i.to_owned())),
        _ => { print_error(ExpectedVariable) }
    };
    stack
}

pub(crate) fn get_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim_end().to_string()
}


pub(crate) fn is_op(el: &str) -> bool {
    QUOTATION_OPS.contains(&el) ||
    IO_OPS.contains(&el) ||
    STACK_OPS.contains(&el) ||
    STRING_OPS.contains(&el) ||
    ARITHMETIC_OPS.contains(&el) ||
    LOGICAL_OPS.contains(&el) ||
    LIST_OPS.contains(&el)
}

pub fn pop_front(t: Type) -> (Option<Type>, Type) {

    match t {

        Block_(val) => {

            let mut new = val.clone();

            new.reverse();
            let el = new.pop();
            new.reverse();

            (el, Block_(new))

        }
        _ => { (None, t) }
    }
}



// Returns the length of the list or string
pub(crate) fn length(stack: &mut Stack<Type>) -> Stack<Type> {
    
    if stack.is_empty() { print_error(StackEmpty); return stack.to_owned(); }

    let mut og = stack.clone();

    let elem = stack.pop().unwrap_or_else(|| String_(String::new()));


    // If it is a list
    if let List_(x) = elem.to_owned() {
        list_op(&mut og.to_owned(), "length", x, String_(String::new()))
    }

    // If it is a code block
    else if is_block(vec![elem.to_owned()]) {
        og.replace_last_match(vec![elem.to_owned()], String_(String::new()));
        quotation(&mut og.to_owned(), "length", elem, List_(vec![]))
    }

    else { parse_string("length", &mut og) }

}


// By making this a separate function, several datatypes can be compared
pub(crate) fn compare(stack: &mut Stack<Type>) -> Stack<Type> {

    let mut elem1 = None;
    let mut elem2 = None;
    let mut is_number = false;
    let mut is_string = false;

    let mut old_stack = stack.clone();

    // Set elem1 and elem2 to be the next 2 numbers or strings in the stack
    loop {

        if let Some(elem) = old_stack.pop() {
            match elem {
                // When the first element that is either a string or a number is found,
                // set elem1 to be the element and set the corresponding boolean to true
                // This ensures that elem1 and elem2 are both either strings or numbers
                Int_(_) | Float_(_) => {
                    if let Some(Int_(_) | Float_(_)) = elem1.to_owned() { elem2 = Some(elem); break }
                    else if !is_string { elem1 = Some(elem); is_number = true; }
                }
                String_(_) => {
                    if let Some(String_(_)) = elem1.to_owned() { elem2 = Some(elem); break }
                    else if !is_number { elem1 = Some(elem); is_string = true; }
                }
                _ => {}
            }
        }
            
        else { break }
    }

    if elem1.is_some() && elem2.is_some() {

        // This ensures that i.e., 10.0 and 10 are considered as equal
        let v1: f64 = elem1.unwrap().type_to_float();
        let v2: f64 = elem2.unwrap().type_to_float();

        stack.push(Bool_(v1 == v2));
    }
    else { print_error(ExpectedNumber); };
    
    stack.to_owned()

}

pub(crate) fn invert(stack: &mut Stack<Type>) -> Stack<Type> {

    let mut old_stack = stack.clone();
    loop {

        if let Some(elem) = old_stack.pop() {
            match elem {

                // Turns a negative number positive, or the opposite
                Int_(el) => {
                    stack.replace_last_match(vec![elem.to_owned()], Int_(el.neg())); break; }
                
                Float_(el) => {
                    stack.replace_last_match(vec![elem.to_owned()], Float_(el.neg())); break; }

                // Inverts the predicate
                Bool_(el) => {
                    let new_elem = if el { Some(Bool_(false)) }
                    else { Some(Bool_(true)) };
                    stack.replace_last_match(vec![elem.to_owned()], new_elem.unwrap()); break;
                }
                _ => {}
            }
        }
        else { break }
    }
    
    stack.to_owned()
}

