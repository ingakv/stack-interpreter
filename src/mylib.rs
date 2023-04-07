
use std::io;
use std::io::{Write};

fn getline() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim_end().to_string()
}

pub fn slay() {

    let mut stack: Vec<String> = Vec::new();

    loop {
        print!("bprog> ");
        io::stdout().flush();

        let input = getline();


        let new_el: Vec<&str> = input.split_whitespace().collect();


        for i in new_el {
            stack = check_for_operator(i, stack.clone());
        }


        println!("Stack: ");
        for i in stack.iter().rev() {
            println!("{}",i);
        }

    }

}

fn check_for_operator(elem : &str, mut stack: Vec<String>) -> Vec<String> {

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


////////////////////// Simple IO //////////////////////////////

        // Prints the top element to standard output
        "print" => {
            if let Some(str_ref) = stack.last() {
            let str_val: String = str_ref.to_owned();
            println!("{}\n", str_val);
            } else {}
        },


        // Reads an input, and adds it to the stack
        "read" => { stack.push(getline()); },


        // If a stack operation was not typed in, push the value to the stack
        _ => stack.push(elem.to_string()),


    }

    // Return the stack
    stack
}

