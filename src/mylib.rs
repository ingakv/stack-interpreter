
use std::{io};



pub(crate) fn push_to_vec<'a>(old_vec: Vec<&'a str>, vec: Vec<&'a str>) -> Vec<&'a str> {

    let mut nvec = old_vec.clone();

    nvec.push( "\"" );
    for el in vec {
        nvec.push(el);
    }
    nvec.push("\"");
    nvec.push(" ");
    nvec

}


pub(crate) fn check_operator(c : &str, stack: &mut Vec<String>) -> Vec<String> {

    match c {

        // Ignores ""
        "" => {stack.to_vec()}

        "dup" | "swap" | "pop" => { stack_op(c, stack) },

        "print" | "read" => { simple_io(c, stack) },

//        "True" | "False" | "not" |

        "&&" | "||" => { logical_op(stack, c) },

        "+" | "-" | "*" | "/" | "div" | "<" | ">" | "==" => {

            // Adds the operator onto the stack
            let mut new = stack.clone();
            new.push(c.to_string());

            let mut new2 = new.clone();


            find_arithmetic(&mut new, &mut new2)
        },

        _ => {


            let lower = c.to_lowercase();

            // Forces bools to have a capitalized first letter
            let new =
                match lower.as_str() {
                    "true" => {"True"}
                    "false" => {"False"}
                    _ => {c}
                };

            // If a stack operation was not typed in, push the value to the stack
            stack.push(new.to_string());
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


fn logical_op(stack: &mut Vec<String>, c:&str) -> Vec<String> {

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


fn find_arithmetic(stack: &mut Vec<String>, og: &mut Vec<String>) -> Vec<String> {


    let c = if stack.is_empty() {
        "".to_string()
    }
    else {
        // Remove top element and store it
        stack.pop().unwrap()
    };

    let ops = ["+", "-", "*", "/", "div", "<", ">", "=="];

    if c == "".to_string() {
        vec![]
    }

    // Checks if it is an operator
    else if ops.contains(&&*c) {
        // Loops through and finds the next two numbers
        let num2 = find_arithmetic(stack, og);
        let num1 = find_arithmetic(stack, og);

        if !num1.is_empty() && !num2.is_empty() {
            arithmetic(og, &c, num1.first().unwrap(), num2.first().unwrap())
        }
        else {
            og.pop();
            og.to_vec()
        }

    }

    else if c.as_bytes()[0].is_ascii_digit() {
        vec![c]
    }

    else if c.contains('-') && c.as_bytes()[1].is_ascii_digit() {
        vec![c]
    }

    else {
        find_arithmetic(stack, og)
    }
}

fn arithmetic(stack: &mut Vec<String>, c:&str, x: &String, y: &String) -> Vec<String> {

    let v1: f64 = x.parse().unwrap();
    let v2: f64 = y.parse().unwrap();

    let new = match c {

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
        "<" => { (if v1 < v2 {"True"} else {"False"}).to_string() },

        // Bigger than
        ">" => { (if v1 > v2 {"True"} else {"False"}).to_string() },

        // Equals
        "==" => { (if v1 == v2 {"True"} else {"False"}).to_string() },

        _ => panic!("Invalid input!")

    };



    // Ensures that if there are duplicates of the numbers, the ones removed are the ones in the back
    stack.reverse();

    if let Some(str_ref) = stack.iter().position(|r| r == x) {
        stack.remove(str_ref);
    }
    if let Some(str_ref) = stack.iter().position(|r| r == y) {
        stack.remove(str_ref);
    }

    // Reverse it back
    stack.reverse();


    // Removes the operator and adds the new variable
    stack.pop();
    stack.push(new);

    stack.to_owned()
}



pub(crate) fn get_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim_end().to_string()
}

