use crate::error_handling::print_error;
use crate::error_handling::Error::{DivisionByZero, ExpectedNumber};
use crate::mylib::is_number;
use crate::structs::Type::{Bool_, Float_, Int_, String_};
use crate::structs::{Stack, Type};

pub(crate) const ARITHMETIC_OPS: [&str; 8] = ["+", "-", "*", "/", "div", "<", ">", "=="];

pub(crate) fn find_arithmetic(
    stack: &mut Stack<Type>,
    og: &mut Stack<Type>,
    skip: bool,
) -> Stack<Type> {
    // Remove the top element and store it
    let c = stack.pop().unwrap_or_else(|| String_(String::new()));

    let st = c.type_to_string();
    let op = st.trim_start_matches("\"").trim_end_matches("\"");

    // Skips if the stack is empty
    if c.is_empty() {
        Stack { elements: vec![] }
    }
    // Checks if it is an operator
    else if ARITHMETIC_OPS.contains(&op) && !skip {
        // Loops through and finds the next two numbers
        let num2 = find_arithmetic(stack, og, true);
        let num1 = find_arithmetic(stack, og, true);

        if let (Some(x), Some(y)) = (num1.first(), num2.first()) {
            arithmetic(og, &op, x, y)
        }
        // If there are less than two valid numbers in the stack, the original stack gets sent back
        // (without the operator)
        else {
            print_error(ExpectedNumber);
            og.pop();
            og.to_owned()
        }
    } else if is_number(op) {
        Stack { elements: vec![c] }
    } else {
        find_arithmetic(stack, og, true)
    }
}

fn arithmetic(stack: &mut Stack<Type>, c: &str, x: Type, y: Type) -> Stack<Type> {
    // Float is set as the default value to do calculations
    let float_x = x.type_to_float();
    let float_y = y.type_to_float();
    let int_x = x.type_to_int();
    let int_y = y.type_to_int();

    let is_float = matches!(x, Float_(_)) || matches!(y, Float_(_));

    // Calculates the answers to the arithmetic operations
    let new_el = match c {
        // Addition
        "+" => {
            if is_float {
                Float_(float_x + float_y)
            } else {
                Int_(int_x + int_y)
            }
        }

        // Subtraction
        "-" => {
            if is_float {
                Float_(float_x - float_y)
            } else {
                Int_(int_x - int_y)
            }
        }

        // Multiplication
        "*" => {
            if is_float {
                Float_(float_x * float_y)
            } else {
                Int_(int_x * int_y)
            }
        }

        // Division
        "/" | "div" => {
            if float_y == 0.0 {
                print_error(DivisionByZero);
                stack.push(x.to_owned());
                stack.push(y.to_owned());
                String_(String::new())
            } else if is_float {
                Float_(float_x / float_y)
            } else { Int_(int_x / int_y) }
        }

        // Smaller than
        "<" => {
            Bool_(float_x < float_y)
        }

        // Bigger than
        ">" => {
            Bool_(float_x > float_y)
        }

        _ => panic!("An error occurred in arithmetic_ops!"),
    };

    // Removes the operator and adds the new variable
    stack.pop();

    // Remove the original numbers or replaces them with the new element
    stack.replace_last_match(vec![x.to_owned(), y.to_owned()], new_el);

    stack.to_owned()
}
