
use std::io;
use std::io::{Write};
use std::num;



pub fn main() {

    let mut stack: Vec<String> = Vec::new();

    loop {
        print!("bprog> ");
        io::stdout().flush().unwrap();

        let input = get_line();


        let new_el: Vec<&str> = input.split_whitespace().collect();


        for i in new_el {
            stack = check_operator(i, &mut stack);
        }

        println!("Stack: ");
        for i in stack.iter().rev() {
            println!("{}",i);
        }

    }

}

fn check_operator(c : &str, stack: &mut Vec<String>) -> Vec<String> {

    match c {
        "dup" | "swap" | "pop" => { stack_op(c, stack) },

        "print" | "read" => { simple_io(c, stack) },

        "+" | "-" | "*" | "/" | "div" | "<" | ">" | "==" => {

            // Adds the operator onto the stack
            let mut new = stack.clone();
            new.push(c.to_string());

            // Adds the answer
            stack.push(find_arithmetic(&mut new));

            // Removes the two original variables
            stack.remove(stack.len()-2);
            stack.remove(stack.len()-2);

            stack.to_vec()
        },

        _ => {
            // If a stack operation was not typed in, push the value to the stack
            stack.push(c.to_string());
            stack.to_vec()
        }
    }
}




fn stack_op(elem : &str, stack: &mut Vec<String>) -> Vec<String> {

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


fn simple_io(elem : &str, stack: &mut Vec<String>) -> Vec<String> {

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



fn find_arithmetic(stack: &mut Vec<String>) -> String {
    let c = stack.pop().unwrap();
    let ops = ["+", "-", "*", "/", "div", "<", ">", "=="];

    if ops.contains(&&*c) {
        let num1 = find_arithmetic(stack);
        let num2 = find_arithmetic(stack);


        arithmetic(&c, num1, num2)

    }
    else {
        c
    }
}

fn arithmetic(c:&str, x: String, y: String) -> String {

    let v1: f64 = x.parse().unwrap();
    let v2: f64 = y.parse().unwrap();

    match c {

        // Calculates the answers to the arithmetic operations

        // Addition
        "+" => { (v1 + v2).to_string() },

        // Subtraction
        "-" => { (v1 - v2).to_string() },

        // Multiplication
        "*" => { (v1 * v2).to_string() },

        // Floating point division
        "/" => { (v1 / v2).to_string() },

        // Integer division
        "div" => {
            let a = v1 as i64;
            let b = v2 as i64;

            (a / b).to_string()
        },

        // Smaller than
        "<" => { (v1 < v2).to_string() },

        // Bigger than
        ">" => { (v1 > v2).to_string() },

        // Equals
        "==" => { (v1 == v2).to_string() },

        _ => panic!("Invalid input!")

    }
}



fn get_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim_end().to_string()
}

