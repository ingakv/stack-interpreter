use crate::error_handling::print_error;
use crate::error_handling::Error::{DivisionByZero, ExpectedNumber};
use crate::stack::Type::{Bool_, Float_, Int_, Variable};
use crate::stack::Type;

pub(crate) const ARITHMETIC_OPS: [&str; 9] = ["+", "-", "*", "/", "div", "<", ">", "<=", ">="];
pub(crate) const LOGICAL_OPS: [&str; 2] = ["&&", "||"];


pub fn arithmetic(c: String, x: Type, y: Type) -> (Vec<Type>, Vec<Type>) {
    // Float is set as the default value to do calculations
    let float_x = x.type_to_float().unwrap_or_else(|| panic!("{:?}", print_error(ExpectedNumber)));
    let float_y = y.type_to_float().unwrap_or_else(|| panic!("{:?}", print_error(ExpectedNumber)));
    let int_x = x.type_to_int().unwrap_or_else(|| panic!("{:?}", print_error(ExpectedNumber)));
    let int_y = y.type_to_int().unwrap_or_else(|| panic!("{:?}", print_error(ExpectedNumber)));

    let is_float = matches!(x, Float_(_)) || matches!(y, Float_(_));

    // Calculates the answers to the arithmetic operations
    let mut new_el = vec![];
    match c.as_str() {
        // Addition
        "+" => {
            new_el.push(if is_float {
                Float_(float_x + float_y)
            } else {
                Int_(int_x + int_y)
            })
        }

        // Subtraction
        "-" => {
            new_el.push(if is_float {
                Float_(float_x - float_y)
            } else {
                Int_(int_x - int_y)
            })
        }

        // Multiplication
        "*" => {
            new_el.push(if is_float {
                Float_(float_x * float_y)
            } else {
                Int_(int_x * int_y)
            })
        }

        // Division
        "/" | "div" => {
            if float_y == 0.0 {
                print_error(DivisionByZero);
                new_el.push(x.to_owned());
                new_el.push(y.to_owned());
            } else if is_float {
                new_el.push(Float_(float_x / float_y));
            } else { new_el.push(Int_(int_x / int_y)); }
        }

        // Smaller than
        "<" => {
            new_el.push(Bool_(float_x < float_y))
        }

        // Smaller than or equal to
        "<=" => {
            new_el.push(Bool_(float_x <= float_y))
        }

        // Bigger than
        ">" => {
            new_el.push(Bool_(float_x > float_y))
        }

        // Bigger than or equal to
        ">=" => {
            new_el.push(Bool_(float_x >= float_y))
        }

        _ => panic!("An error occurred in arithmetic_ops!"),
    };

    // Return the operator, the original numbers and the new element
    (vec![Variable(c), x, y], new_el)
}


pub fn logical_op(c: String, x: bool, y: bool) -> (Vec<Type>, Vec<Type>) {
    let new = match c.as_str() {
        // Checks whether both predicates are True or not
        "&&" => x && y,

        // Checks whether at least one of the predicates is True or not
        "||" => x || y,

        _ => panic!("An error occurred in logical_ops!"),
    };
    
    (vec![Variable(c), Bool_(x), Bool_(y)], vec![Bool_(new)])
}

