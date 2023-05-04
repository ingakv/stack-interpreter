
use crate::error_handling::{print_error};
use crate::error_handling::Error::{DivisionByZero, ExpectedNumber};
use crate::mylib::{is_float, is_number};
use crate::structs::{Stack, Type};
use crate::structs::Type::{Bool_, Float_, Int_, String_};

pub(crate) const ARITHMETIC_OPS: [&str; 7] = ["+", "-", "*", "/", "div", "<", ">"];


pub(crate) fn find_arithmetic(stack: &mut Stack<Type>, og: &mut Stack<Type>) -> Stack<Type> {

    // Remove top element and store it
    let c = stack.pop().unwrap_or_else(|| String_("".to_string()));

    let st = c.type_to_string();
    let op = st.trim_start_matches("\"").trim_end_matches("\"");

    // Skips if the stack is empty
    if c == String_("".to_string()) {
        Stack{ elements: vec![] }
    }

    // Checks if it is an operator
    else if ARITHMETIC_OPS.contains(&op) {
        // Loops through and finds the next two numbers
        let num2 = find_arithmetic(stack, og);
        let num1 = find_arithmetic(stack, og);

        if let (Some(x), Some(y)) = (num1.first(), num2.first()) {
            arithmetic(og, &op, x, y)
        }

        // If there are less than two valid numbers in the stack, the original stack gets sent back
        // (without the operator)
        else {
            print_error(ExpectedNumber);
            og.pop();
            og.clone()
        }

    }


    else if is_number(op) {
        Stack{ elements: vec![c] }
    }

    else {
        find_arithmetic(stack, og)
    }
}


fn arithmetic(stack: &mut Stack<Type>, c: &str, x: Type, y: Type) -> Stack<Type> {

    // Float is set as the default value to do calculations
    let v1 = x.type_to_float();
    let v2 = y.type_to_float();
    let a = x.type_to_int();
    let b = y.type_to_int();

    let is_float = is_float(x.type_to_string().as_str()) || is_float(y.type_to_string().as_str());


    // Calculates the answers to the arithmetic operations
    let new_el = match c {

        // Addition
        "+" => if is_float {Float_(v1 + v2)} else { Int_((v1 as i128) + (v2 as i128)) },

        // Subtraction
        "-" => if is_float {Float_(v1 - v2)} else { Int_((v1 as i128) - (v2 as i128)) },

        // Multiplication
        "*" => if is_float {Float_(v1 * v2)} else { Int_((v1 as i128) * (v2 as i128)) },

        // Floating point division
        "/" => {
            if v2 == 0.0 {
                print_error(DivisionByZero);
                stack.push(x.clone());
                stack.push(y.clone());
                String_("".to_string())
            }
            else { Float_(v1 / v2) }
        },

        // Integer division
        "div" => {
            if b == 0 {
                print_error(DivisionByZero);
                stack.push(x.clone());
                stack.push(y.clone());
                String_("".to_string())
            }
            else { Int_(a / b) }
        },

        // Smaller than
        "<" => if v1 < v2 { Bool_(true) } else { Bool_(false) },

        // Bigger than
        ">" => if v1 > v2 { Bool_(true) } else { Bool_(false) },

        _ => panic!("An error occurred in arithmetic_ops!"),
    };


    // Remove the original numbers
    stack.remove_last_match(x.clone());
    stack.remove_last_match(y.clone());


    // Removes the operator and adds the new variable
    stack.pop();
    if new_el != String_("".to_string()) { stack.push(new_el); }

    stack.to_owned()
}
