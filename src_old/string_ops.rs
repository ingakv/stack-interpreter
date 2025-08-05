use crate::error_handling::Error::{ExpectedNumber, ExpectedString};
use crate::error_handling::print_error;
use crate::mylib::{get_line, is_number};

pub(crate) const STACK_OPS: [&str; 3] = ["dup", "swap", "pop"];

pub(crate) const IO_OPS: [&str; 2] = ["print", "read"];

pub(crate) const STRING_OPS: [&str; 3] = ["parseInteger", "parseFloat", "words"];

pub(crate) fn parse_string(elem: &str, stack: &mut Vec<String>) -> Vec<String> {

    match elem {
        // Converts a string to an integer
        "parseInteger" => {
            if let Some(str_ref) = stack.pop() {
                let str_val = str_ref.trim_matches('\"').to_string();
                if is_number(str_val.clone()) {
                    stack.push(str_val);
                }
                else { print_error(ExpectedNumber); stack.push(str_ref); }
            } else { print_error(ExpectedString)
            }
        }

        // Converts a string to a float
        "parseFloat" => {
            if let Some(str_ref) = stack.pop() {
                let str_val: f64 = str_ref.trim_matches('\"').parse().unwrap();
                stack.push(str_val.to_string());
            } else {
            }
        }

        // Divides the string into words, and puts them in a list
        "words" => {
            if let Some(str_ref) = stack.pop() {
                let str_val: Vec<&str> = str_ref.split_whitespace().collect();

                let mut new_li = vec!["["];

                for i in str_val {
                    new_li.push("\"");
                    new_li.push(i.trim_matches('\"'));
                    new_li.push("\"");
                    new_li.push(",");
                }
                new_li.pop();
                new_li.push("]");

                stack.push(new_li.concat());
            } else {
            }
        }

        // Returns the length of the string
        "length" => {
            let mut top = stack.last().unwrap().clone();

            top = top.trim_matches('\"').to_string();

            stack.push(top.len().to_string())
        },

        _ => {}
    }

    // Return the stack
    stack.to_owned()
}

pub(crate) fn stack_op(elem: &str, stack: &mut Vec<String>) -> Vec<String> {
    match elem {
        // dup duplicates the top element
        "dup" => {
            if let Some(str_ref) = stack.last() {
                let str_val: String = str_ref.to_owned();
                stack.push(str_val);
            } else {
            }
        }

        // swap swaps the top two elements
        "swap" => {
            let len = stack.len();
            if len > 1 {
                stack.swap(len - 2, len - 1);
            } else {
            }
        }

        // pop removes the top element
        "pop" => {
            stack.pop();
        }

        _ => {}
    }

    // Return the stack
    stack.to_owned()
}

pub(crate) fn simple_io(elem: &str, stack: &mut Vec<String>) -> Vec<String> {
    match elem {
        // Prints the top element to standard output
        "print" => {
            if let Some(str_ref) = stack.pop() {
                let str_val: String = str_ref.to_owned();
                println!("{}\n", str_val);
            } else {}
        },


        // Reads an input, and adds it to the stack
        "read" => { stack.push(get_line()); },

        _ => {}
    }

    // Return the stack
    stack.to_owned()
}

pub(crate) fn find_string(stack: &mut Vec<String>) -> Vec<String> {

    let c = if stack.is_empty() {
        "".to_string()
    }

    else {
        // Remove top element and store it
        stack.pop().unwrap()
    };

    // Skips if the stack is empty
    if c == "".to_string() {
        vec![]
    }

    else if c.contains("\"") && !c.contains("[") {
        vec![c]
    }

    else if is_number(c.clone()) {
        vec![c]
    }

    else {
        find_string(stack)
    }
}
