use crate::error_handling::print_error;
use crate::error_handling::Error::{DivisionByZero, ExpectedNumber};
use crate::stack::Type::{Bool_, Float_, Int_, String_, Variable};
use crate::stack::{Stack, Type};

pub(crate) const ARITHMETIC_OPS: [&str; 9] = ["+", "-", "*", "/", "div", "<", ">", "<=", ">="];
pub(crate) const LOGICAL_OPS: [&str; 2] = ["&&", "||"];


pub fn arithmetic(stack: &mut Stack<Type>, c: String, x: Type, y: Type) -> Stack<Type> {
    // Float is set as the default value to do calculations
    let float_x = x.type_to_float().unwrap_or_else(|| panic!("{:?}", print_error(ExpectedNumber)));
    let float_y = y.type_to_float().unwrap_or_else(|| panic!("{:?}", print_error(ExpectedNumber)));
    let int_x = x.type_to_int().unwrap_or_else(|| panic!("{:?}", print_error(ExpectedNumber)));
    let int_y = y.type_to_int().unwrap_or_else(|| panic!("{:?}", print_error(ExpectedNumber)));

    let is_float = matches!(x, Float_(_)) || matches!(y, Float_(_));

    // Calculates the answers to the arithmetic operations
    let new_el = match c.as_str() {
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

        // Smaller than or equal to
        "<=" => {
            Bool_(float_x <= float_y)
        }

        // Bigger than
        ">" => {
            Bool_(float_x > float_y)
        }

        // Bigger than or equal to
        ">=" => {
            Bool_(float_x >= float_y)
        }

        _ => panic!("An error occurred in arithmetic_ops!"),
    };

    // Remove the operator, the original numbers or replaces them with the new element
    stack.replace_last_match(vec![Variable(c), x, y], new_el);

    stack.to_owned()
}


pub fn logical_op(stack: &mut Stack<Type>, c: String, x: bool, y: bool) -> Stack<Type> {
    let new = match c.as_str() {
        // Checks whether both predicates are True or not
        "&&" => x && y,

        // Checks whether at least one of the predicates is True or not
        "||" => x || y,

        _ => panic!("An error occurred in logical_ops!"),
    };

    // Removes the operator and adds the new variable
    stack.replace_last_match(vec![Variable(c), Bool_(x), Bool_(y)], Bool_(new));

    // Return the stack
    stack.to_owned()
}

