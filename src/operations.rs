use crate::mylib::{get_line};



pub(crate) fn parse_string(elem : &str, stack: &mut Vec<String>) -> Vec<String> {

    match elem {

        // Converts a string to an integer
        "parseInteger" => {
            if let Some(str_ref) = stack.pop() {
                let str_val: i64 = str_ref.parse().unwrap();
                stack.push(str_val.to_string());
            } else {}
        },

        // Converts a string to a float
        "parseFloat" => {
            if let Some(str_ref) = stack.pop() {
                let str_val: f64 = str_ref.parse().unwrap();
                stack.push(str_val.to_string());
            } else {}
        },

        // Converts a string to a float
        "words" => {
            if let Some(str_ref) = stack.pop() {
                let str_val: Vec<&str> = str_ref.split_whitespace().collect();

                let mut new_li = vec!["[ "];

                for i in str_val {
                    new_li.push("\"");

                    new_li.push(i.trim_matches('\"'));
                    new_li.push("\" ");
                }

                new_li.push("]");


                stack.push(new_li.concat());
            } else {}
        },

        _ => {}

    }

    // Return the stack
    stack.to_owned()
}


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

