
use crate::mylib::get_line;

pub(crate) fn stack_op(elem : &str, stack: &mut Vec<String>) -> Vec<String> {

    match elem {

        // dup duplicates the top element
        "dup" => {
            if let Some(str_ref) = stack.last() {
                let str_val: String = str_ref.to_owned();
                stack.push(str_val);
            } else {}
        },

        // swap swaps the top two elements
        "swap" => {
            let len = stack.len();
            if len > 1 {stack.swap(len-2, len-1);} else {}
        },

        // pop removes the top element
        "pop" => {stack.pop();},

        _ => {}

    }

    // Return the stack
    stack.to_owned()
}


pub(crate) fn simple_io(elem : &str, stack: &mut Vec<String>) -> Vec<String> {

    match elem {

        // Prints the top element to standard output
        "print" => {
            if let Some(str_ref) = stack.last() {
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


pub(crate) fn logical_op(stack: &mut Vec<String>, c:&str) -> Vec<String> {

    let x = if stack.is_empty() {"".to_string()}
    else {
        // Remove top element and store it
        stack.pop().unwrap()
    };

    let y = if stack.is_empty() {"".to_string()}
    else {
        // Remove top element and store it
        stack.pop().unwrap()
    };

    // If there is only 1 variable, it gets pushed back on, and the stack returns
    // If there are none the stack returns without any changes

    if x.is_empty() {}
    else if y.is_empty() { stack.push(x); }

    else {
        match c {

            // Checks whether both predicates are True or not
            "&&" => {
                if x == "True" && y == "True" { stack.push("True".to_string()) } else { stack.push("False".to_string()) }
            },

            // Checks whether at least one of the predicates are True or not
            "||" => {
                if x == "True" || y == "True" { stack.push("True".to_string()) } else { stack.push("False".to_string()) }
            },

            _ => {}
        }
    }

    // Return the stack
    stack.to_owned()
}