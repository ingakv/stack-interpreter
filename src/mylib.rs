
use crate::arithmetic_ops::{find_arithmetic, ARITHMETIC_OPS};
use crate::list_ops::{find_list, list_op, LIST_OPS};
use crate::logical_ops::{find_logical, LOGICAL_OPS};
use crate::string_ops::{parse_string, simple_io, stack_op, IO_OPS, STACK_OPS, STRING_OPS};
use std::io;
use std::io::{Write};
use crate::error_handling::Error::{ExpectedListOrString, ExpectedNumber, ProgramFinishedWithMultipleValues, StackEmpty};
use crate::error_handling::{print_error};


pub(crate) fn normal() {
    let mut stack: Vec<String> = Vec::new();

    loop {

        print!("\n:q to quit\n:s to print the stack\nbprog> ");
        io::stdout().flush().unwrap();

        // Reads user input

        let input = get_line();

        if input == ":q" {
            // Ctrl + D pressed, execute your code here
            let mut new = stack.clone();
            new.retain(|x| !x.contains(","));

            let new2 = new.into_iter().flat_map(|i| [i, " ".to_string()]).collect();

            stack = program_loop(new2, vec![], true);


            // Prints the result of the operations
            if stack.is_empty() { print_error(StackEmpty); }
            else {

                let result = stack.pop();
                println!("\n{}", result.unwrap());

                if !stack.is_empty() {
                    // Prints the stack
                    print_error(ProgramFinishedWithMultipleValues);
                    println!("Stack: ");
                    for i in stack.iter().rev() {
                        println!("{}", i);
                    }
                }
                else { print_error(StackEmpty); }

            }
            break;
        }

        // Prints the stack
        else if input == ":s" {

            println!("Stack: ");
            if !stack.is_empty() {
                for i in stack.iter().rev() {
                    println!("{}", i);
                }
            }
            else { print_error(StackEmpty); }
        }

        else {
            stack = program_loop(input, stack.clone(), false);
        }



    }
}




// REPL mode (looping through and executing the code for each user input)
pub(crate) fn repl() {
    let mut stack: Vec<String> = Vec::new();

    loop {
        print!("\nbprog> ");
        io::stdout().flush().unwrap();

        // Reads user input
        let input = get_line();

        stack = program_loop(input, stack.clone(), true);

        // Prints the stack
        println!("Stack: ");
        for i in stack.iter().rev() {
            println!("{}", i);
        }

    }
}


pub fn program_loop(input: String, mut stack: Vec<String>, repl: bool) -> Vec<String> {

    // Splits up the different input variables
    let new_el: Vec<&str> = { input.split_whitespace().collect() };

    if new_el.is_empty() { print_error(StackEmpty); }

    // Variables to help join the elements together
    let mut str_buf: Vec<&str> = vec![];
    let mut is_str: bool = false;

    let mut li_buf: Vec<&str> = vec![];
    let mut is_list: bool = false;

    for i in new_el {

        //////////////// String /////////////////


        // If it is the start or the end of a string
        if i.contains('"') {
            // If it is the end of the string
            if is_str {
                // Remove the last whitespace
                str_buf.pop();

                // Copy the elements into a combined list
                // If there is no list, there are no extra elements added, so str_buf can get set to the new list

                str_buf = push_to_vec(li_buf.clone(), str_buf.clone());

                // If we are in a list, copy the new list over
                if is_list {
                    li_buf = str_buf.clone();
                    li_buf.push(", ")
                }

                // Join the vector together to form a sentence / string, and send it to the stack
                else {
                    if repl { stack = check_operator(str_buf.concat().as_str(), &mut stack); }

                    else {
                        for item in str_buf.clone() {
                            stack.push(item.to_string())
                        }
                    }
                }

                // Reset the buffer so that a potential new string can be read
                str_buf.clear();

            }

            // Flip the boolean
            is_str = !is_str;
        }



        // If a string is currently being read, push it to the buffer, with a whitespace after
        else if is_str {
            str_buf.push(i);
            str_buf.push(" ");
        }


        //////////////// List /////////////////


        // If it is the start or the end of a list
        else if i.contains('[') {
            is_list = true;

            // Add opening bracket
            li_buf.push("[");
        }


        // If it is the end of the list
        else if i.contains(']') {

            if li_buf.last().unwrap() != &"[" {
                // Remove the last comma
                li_buf.pop();
            }

            // Add closing bracket
            li_buf.push("]");

            // If the list is not a sublist, set is_list to false
            if li_buf.iter().filter(|&n| *n == "[").count()
                == li_buf.iter().filter(|&n| *n == "]").count()
            {
                // Join the vector together to form a list, and send it to the stack
                if repl { stack = check_operator(li_buf.concat().as_str(), &mut stack); }

                else {
                    for item in li_buf.clone() {
                        stack.push(item.to_string())
                    }
                }

                is_list = false;

                // Reset the buffer so that a potential new list can be read
                li_buf.clear();
            }
            // If the list is a sublist, continue reading it
            else {
                li_buf.push(",");
            }
        }

        // If a list is currently being read, push it to the buffer, with a comma after
        else if is_list {
            li_buf.push(i);
            li_buf.push(",");
        }

        else {
            if repl { stack = check_operator(i, &mut stack); }
            else { stack.push(i.to_string()); }

        }
    }

    stack

}



fn check_operator(c: &str, stack: &mut Vec<String>) -> Vec<String> {

    // Ignores ""
    if c == "" { stack.to_vec() }

    else if c == "==" {

        if stack.len() > 1 {
            compare(stack)
        }

        else { print_error(ExpectedNumber); stack.to_vec() }

    }


    else if c == "length" {

        if !stack.is_empty() {
            length(stack)
        }

        else { print_error(ExpectedListOrString); stack.to_vec() }

    }

    else if ARITHMETIC_OPS.contains(&c) {
        // Adds the operator onto the stack
        let mut new = stack.clone();
        new.push(c.to_string());

        let mut new2 = new.clone();

        find_arithmetic(&mut new, &mut new2)
    }

    else if LOGICAL_OPS.contains(&c) {
        // Adds the operator onto the stack
        let mut new = stack.clone();
        new.push(c.to_string());

        let mut new2 = new.clone();

        find_logical(&mut new, &mut new2)
    }

    else if STRING_OPS.contains(&c) {
        parse_string(c, stack)
    }

    else if LIST_OPS.contains(&c) {
        // Adds the operator onto the stack
        let mut new = stack.clone();
        new.push(c.to_string());

        let mut new2 = new.clone();

        find_list(&mut new, &mut new2)
    }


    else if STACK_OPS.contains(&c) { stack_op(c, stack) }

    else if IO_OPS.contains(&c) { simple_io(c, stack) }

    else {

        // Forces bools to have a capitalized first letter
        let new = match c.to_lowercase().as_str() {
            "true" => "True",
            "false" => "False",
            _ => c,
        };

        // If a stack operation was not typed in, push the value to the stack
        stack.push(new.to_string());
        stack.to_vec()
    }
}

pub(crate) fn get_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim_end().to_string()
}

pub(crate) fn push_to_vec<'a>(old_vec: Vec<&'a str>, vec: Vec<&'a str>) -> Vec<&'a str> {
    let mut nvec = old_vec.clone();

    nvec.push("\"");
    for el in vec {
        nvec.push(el);
    }
    nvec.push("\"");
    nvec
}

pub(crate) fn is_literal(el: String) -> bool {
    return if el == "True" || el == "False" {
        true
    } else {
        false
    };
}

pub(crate) fn is_number(el: String) -> bool {
    return if el.as_bytes()[0].is_ascii_digit()
        || (el.contains('-') && el.as_bytes()[1].is_ascii_digit())
    {
        true
    } else {
        false
    };
}



// Returns the length of the list or string
pub(crate) fn length(stack: &mut Vec<String>) -> Vec<String> {

    let mut og = stack.clone();

    let elem = stack.pop().unwrap();

    let top = elem.split_at(1).0;

    if top == "\"" { parse_string("length", &mut og) }
    else if top == "[" { list_op(&mut og.clone(), "length", og.first().unwrap(), &"".to_string()) }
    else { print_error(ExpectedListOrString); og.to_owned() }
}


// By making this a separate function, several datatypes can be compared
pub(crate) fn compare(stack: &mut Vec<String>) -> Vec<String> {

    let num1 = stack.pop().unwrap();
    let num2 = stack.pop().unwrap();

    // This ensures that ie 10.0 and 10 is considered as equal
    let ans = if is_number(num1.clone()) && is_number(num2.clone()) {
        let v1: f64 = num1.parse().unwrap();
        let v2: f64 = num2.parse().unwrap();

        (if v1 == v2 { "True" } else { "False" }).to_string()
    }

    else { (if num1 == num2 { "True" } else { "False" }).to_string() };


    stack.push(ans);
    stack.to_owned()

}

