
use crate::arithmetic_ops::{find_arithmetic, ARITHMETIC_OPS};
use crate::list_ops::{find_list, list_op, LIST_OPS};
use crate::logical_ops::{find_logical, LOGICAL_OPS};
use crate::string_ops::{parse_string, simple_io, stack_op, IO_OPS, STACK_OPS, STRING_OPS};
use std::io;
use std::io::{Write};
use crate::error_handling::Error::{ExpectedBool, ExpectedListOrString, ExpectedNumber, ProgramFinishedWithMultipleValues, StackEmpty};
use crate::error_handling::{print_error};
use crate::structs::{Stack, Type};
use crate::structs::Type::{Bool_, Float_, Int_, List_, String_};


pub fn normal() {
    let mut stack: Stack<Type> = Stack::new();

    loop {

        print!("\n:q to quit\n:s to print the stack\nbprog> ");
        io::stdout().flush().unwrap();

        // Reads user input

        let input = get_line();

        if input == ":q" {
            // Ctrl + D pressed, execute your code here
            let new = stack.clone();

            let new2 = new.stack_to_string();

            stack = program_loop(new2, Stack{ elements: vec![] }, true);


            // Prints the result of the operations
            if stack.is_empty() { print_error(StackEmpty); }
            else {

                let result = stack.pop().unwrap_or_else(|| String_("".to_string()));
                result.print();

                stack.print_stack();

            }
            break;
        }

        // Prints the stack
        else if input == ":s" {

            stack.print_stack();
        }

        else {
            if stack.len() > 1 { print_error(ProgramFinishedWithMultipleValues); }
            else { stack = program_loop(input, stack.clone(), false); }
        }

    }
}




// REPL mode (looping through and executing the code for each user input)
pub fn repl() {
    let mut stack: Stack<Type> = Stack::new();

    loop {
        print!("\nbprog> ");
        io::stdout().flush().unwrap();

        // Reads user input
        let input = get_line();

        stack = program_loop(input, stack.clone(), true);

        // Prints the stack
        stack.print_stack();

    }
}


pub fn program_loop(input: String, mut stack: Stack<Type>, repl: bool) -> Stack<Type> {

    // Splits up the different input variables
    let new_el: Vec<&str> = { input.split_whitespace().collect() };

    if new_el.is_empty() { print_error(StackEmpty); }

    // Variables to help join the elements together
    let mut str_buf: Vec<&str> = vec![];
    let mut is_str: bool = false;

    let mut li_buf: Vec<Type> = vec![];
    let mut is_list: bool = false;

    let mut is_sublist = false;
    let mut sublist: Vec<Type> = vec![];

    for i in new_el {

        //////////////// String /////////////////


        // If it is the start or the end of a string
        if i.contains('"') {
            // If it is the end of the string
            if is_str {
                // Remove the last whitespace
                str_buf.pop();


                // If we are in a list, copy the new list over
                if is_list {
                    li_buf.push(String_(str_buf.concat()));
                }

                // Join the vector together to form a sentence / string, and send it to the stack
                else {
                    if repl { stack = check_operator(str_buf.concat().as_str(), &mut stack); }

                    else { stack.push(String_(str_buf.concat().to_string())) }
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

                // Join the vector together to form a list, and send it to the stack
                if repl { stack.push(List_(li_buf.to_owned())); }

                else { stack.push(List_(li_buf.to_owned())) }

                is_list = false;

                // Reset the buffer so that a potential new list can be read
                li_buf.clear();

            }

            else {
                li_buf.push(List_(sublist.to_owned()));
                sublist.clear();
                is_sublist = false;
            }
        }

        // If a sublist is currently being read, push it to the buffer, with a comma after
        else if is_sublist {

            match string_to_type(i) {
                Int_(i) => sublist.push(Int_(i.to_owned())),
                Float_(i) => sublist.push(Float_(i.to_owned())),
                Bool_(i) => sublist.push(Bool_(i.to_owned())),
                String_(i) => sublist.push(String_(i.to_owned())),
                _ => {}
            };
        }

        // If it is not a sublist, push the element to the regular list
        else if is_list {

            match string_to_type(i) {
                Int_(i) => li_buf.push(Int_(i.to_owned())),
                Float_(i) => li_buf.push(Float_(i.to_owned())),
                Bool_(i) => li_buf.push(Bool_(i.to_owned())),
                String_(i) => li_buf.push(String_(i.to_owned())),
                _ => {}
            };
        }

        else {
            if repl { stack = check_operator(i, &mut stack); }
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
    }

    stack

}




fn check_operator(c: &str, stack: &mut Stack<Type>) -> Stack<Type> {

    // Ignores ""
    if c == "" { stack.clone() }

    else if c == "==" {

        if stack.len() > 1 {
            compare(stack)
        }

        else { print_error(ExpectedNumber); stack.clone() }

    }


    else if c == "length" {

        if !stack.is_empty() {
            length(stack)
        }

        else { print_error(ExpectedListOrString); stack.clone() }

    }

    else if ARITHMETIC_OPS.contains(&c) {
        // Adds the operator onto the stack
        let mut new = stack.clone();
        new.push(String_(c.to_string()));

        let mut new2 = new.clone();

        find_arithmetic(&mut new, &mut new2)
    }

    else if LOGICAL_OPS.contains(&c) {
        // Adds the operator onto the stack
        let mut new = stack.clone();
        new.push(String_(c.to_string()));

        let mut new2 = new.clone();

        find_logical(&mut new, &mut new2)
    }

    else if STRING_OPS.contains(&c) {
        parse_string(c, stack)
    }

    else if LIST_OPS.contains(&c) {
        // Adds the operator onto the stack
        let mut new = stack.clone();
        new.push(String_(c.to_string()));

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
        stack.push(string_to_type(new));

        stack.to_owned()
    }
}



// Chooses which type to put the variable in
pub fn string_to_type(var: &str) -> Type {

    if is_float(var) {Float_(var.parse::<f64>().unwrap())}

    else if is_number(var) {Int_(var.parse::<i128>().unwrap())}

    else if is_literal(var) {
        if var == "True" { return Bool_(true); }
        else if var == "False" { return Bool_(false); }
        else { print_error(ExpectedBool); String_("".to_owned()) }

    }

    else {String_(var.to_owned())}
}




pub(crate) fn get_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim_end().to_string()
}


pub(crate) fn is_literal(el: &str) -> bool {
    return el == "True" || el == "False"
}

// Checks whether or not the variable is a valid number
// Returns true for both ints and floats
pub(crate) fn is_number(el: &str) -> bool {
    let mut is_num = true;

    let st: String =  el.split_terminator('.').collect();

    for i in st.trim_start_matches('-').as_bytes() {
        if !i.is_ascii_digit() {is_num = false}
    }
    is_num
}

// Checks whether or not the variable is a float
pub(crate) fn is_float(el: &str) -> bool {
    is_number(el) && el.contains('.')
}


// Checks whether or not the variable is a bool
pub(crate) fn is_bool(el: Type) -> bool {
    el == Bool_(true) || el == Bool_(false)
}

// Checks whether or not the variable is a string
pub(crate) fn is_string(el: Type) -> bool {
    el.type_to_string().contains("\"")
}

// Checks whether or not the variable is a list
pub(crate) fn is_list(el: Type) -> bool {
    el.type_to_string().contains("[") && el.type_to_string().contains("]")
}


// Turns a negative number positive, or the opposite
pub(crate) fn invert_number(el: &str) -> Type {

    let new_number =
        if is_number(el) {
            if el.contains('-') {el.trim_start_matches(|x| x != '-').parse().unwrap()}
            else { let new = vec!["-", el]; new.concat().parse().unwrap() }
        }
        else { print_error(ExpectedNumber); 0 };

    Int_(new_number)

}



// Returns the length of the list or string
pub(crate) fn length(stack: &mut Stack<Type>) -> Stack<Type> {

    let mut og = stack.clone();

    let elem = stack.pop().unwrap_or_else(|| String_("".to_string()));


    // If it is a string
    if is_string(elem.to_owned()) { parse_string("length", &mut og) }


    // If it is a list
    else if let List_(x) = elem.to_owned() {

        og.remove_last_match(elem.to_owned());

        list_op(&mut og.to_owned(), "length", x, String_("".to_owned()))

    }


    else {print_error(ExpectedListOrString); og}

}


// By making this a separate function, several datatypes can be compared
pub(crate) fn compare(stack: &mut Stack<Type>) -> Stack<Type> {

    let num1 = stack.pop().unwrap_or_else(|| String_("".to_string()));
    let num2 = stack.pop().unwrap_or_else(|| String_("".to_string()));

    let ans = if is_number(num1.type_to_string().as_str()) && is_number(num2.type_to_string().as_str()) {

        // This ensures that ie 10.0 and 10 is considered as equal
        let v1: f64 = num1.type_to_float();
        let v2: f64 = num2.type_to_float();

        v1 == v2
    }

    else { num1 == num2 };


    stack.push(Bool_(ans));
    stack.clone()

}

