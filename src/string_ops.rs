use crate::error_handling::Error::{ExpectedList, ExpectedString};
use crate::error_handling::print_error;
use crate::mylib::{get_line, string_to_type};
use crate::structs::{Stack, Type};
use crate::structs::Type::{Float_, Int_, List_, String_};

pub(crate) const STACK_OPS: [&str; 3] = ["dup", "swap", "pop"];

pub(crate) const IO_OPS: [&str; 2] = ["print", "read"];

pub(crate) const STRING_OPS: [&str; 3] = ["parseInteger", "parseFloat", "words"];

pub(crate) fn parse_string(elem: &str, stack: &mut Stack<Type>) -> Stack<Type> {

    match elem {
        // Converts a string to an integer
        "parseInteger" => {
            if let Some(str_ref) = stack.pop() { stack.push(Int_(str_ref.type_to_int()))}
            else { print_error(ExpectedString) }
        }

        // Converts a string to a float
        "parseFloat" => {
            if let Some(str_ref) = stack.pop() { stack.push(Float_(str_ref.type_to_float()))}
            else { print_error(ExpectedString) }
        }

        // Divides the string into words, and puts them in a list
        "words" => {

            if let Some(String_(str_ref)) = stack.pop() {

                let str_val: Vec<&str> = str_ref.split_whitespace().collect();

                let mut new_li = vec![];

                for i in str_val {
                    new_li.push(string_to_type(i));
                }

                stack.push(List_(new_li));
            }
            else { print_error(ExpectedString) }
        }

        // Returns the length of the string
        "length" => {
            if let Some(String_(str_ref)) = stack.last() { stack.push(Int_(str_ref.len() as i128)) }
            else { print_error(ExpectedString) }
        }


        _ => {}
    }

    // Return the stack
    stack.to_owned()
}

pub(crate) fn stack_op(elem: &str, stack: &mut Stack<Type>) -> Stack<Type> {
    match elem {
        // dup duplicates the top element
        "dup" => {
            if let Some(str_ref) = stack.last() { stack.push(str_ref)}
            else { print_error(ExpectedString) }
        }


        // swap swaps the top two elements
        "swap" => {

            let len = stack.len();

            if len > 1 {
                stack.swap(len - 2, len - 1);
            }
            else { print_error(ExpectedList) }


        }

        // pop removes the top element
        "pop" => { stack.pop(); }

        _ => {}
    }

    // Return the stack
    stack.to_owned()
}

pub(crate) fn simple_io(elem: &str, stack: &mut Stack<Type>) -> Stack<Type> {
    match elem {
        // Prints the top element to standard output
        "print" => {

            if let Some(str_ref) = stack.pop() {str_ref.print()}
            else { print_error(ExpectedString) }
        },


        // Reads an input, and adds it to the stack
        "read" => { let input = get_line(); stack.push(string_to_type(input.as_str())); },

        _ => {}
    }

    // Return the stack
    stack.to_owned()
}

pub(crate) fn find_string(stack: &mut Stack<Type>) -> Stack<Type> {

    let c = if stack.is_empty() {
        String_("".to_string())
    }

    else {
        // Remove top element and store it
        stack.pop().unwrap_or_else(|| String_("".to_string()))
    };

    // Skips if the stack is empty
    if c == String_("".to_string()) {
        Stack{ elements: vec![] }
    }

    else if c.is_string() {
        Stack{ elements: vec![c] }
    }

    else if c.is_number() {
        Stack{ elements: vec![c] }
    }

    else {
        find_string(stack)
    }
}
