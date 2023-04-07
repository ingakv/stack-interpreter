
use std::io;
use std::io::{Write};

pub fn slay() {

    let mut stack: Vec<String> = Vec::new();

    loop {
        print!("bprog> ");
        io::stdout().flush();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let newEl: Vec<&str> = input.split_whitespace().collect();


        println!("Stack: ");
        for i in newEl {
            stack = checkForOperator(i, stack.clone());
        }


        for i in stack.iter().rev() {
            println!("{}",i);
        }

    }

}


fn checkForOperator(elem : &str, mut stack: Vec<String>) -> Vec<String> {

    // dup duplicates the top element
    if elem == "dup" {
        if let Some(str_ref) = stack.last() {
            let str_val: String = str_ref.to_owned();
            stack.push(str_val);

        } else {}
    }

    // swap swaps the top two elements
    else if elem == "swap" {
        let len = stack.len();
        if len > 1 {stack.swap(len-1, len);} else {}
    }

    // pop removes the top element
    else if elem == "pop" {
        stack.pop();
    }

    // If a stack operation was not typed in, push the value to the stack
    else {
        stack.push(elem.to_string());
    }

    // Return the stack
    stack
}


