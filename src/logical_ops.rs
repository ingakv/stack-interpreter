use crate::error_handling::print_error;
use crate::error_handling::Error::{DivisionByZero, ExpectedNumber};
use crate::stack::Operators::{And, Div, DivSlash, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual, Minus, Multiply, Or, Plus};
use crate::stack::Type::{Bool_, Float_, Int_};
use crate::stack::{Operators, Type};

pub(crate) fn arithmetic_ops(input: String) -> Option<Operators> {
    let res = match input.as_str() {
        "+" => {Plus},
        "-" => {Minus},
        "*" => {Multiply},
        "/" => {DivSlash},
        "div" => {Div},
        "<" => {LessThan},
        ">" => {GreaterThan},
        "<=" => {LessThanOrEqual},
        ">=" => {GreaterThanOrEqual},
        _ => {return None;}
    };
    Some(res)
}

pub(crate) fn logical_ops(input: String) -> Option<Operators> {
    let res = match input.as_str() {
        "&&" => {And},
        "||" => {Or},
        _ => {return None;}
    };
    Some(res)
}


pub fn arithmetic(c: Operators, x: Type, y: Type) -> (Vec<Type>, Vec<Type>) {

    let is_float = matches!(x, Float_(_)) || matches!(y, Float_(_));

    // Calculates the answers to the arithmetic operations
    let mut new_el = vec![];

    // Float is set as the default value to do calculations
    if let (Some(float_x), Some(float_y), Some(int_x), Some(int_y)) =
        (x.type_to_float(), y.type_to_float(), x.type_to_int(), y.type_to_int() ) {
        
        match c {
            // Addition
            Plus => {
                new_el.push(if is_float {
                    Float_(float_x + float_y)
                } else {
                    Int_(int_x + int_y)
                })
            }

            // Subtraction
            Minus => {
                new_el.push(if is_float {
                    Float_(float_x - float_y)
                } else {
                    Int_(int_x - int_y)
                })
            }

            // Multiplication
            Multiply => {
                new_el.push(if is_float {
                    Float_(float_x * float_y)
                } else {
                    Int_(int_x * int_y)
                })
            }

            // Division
            DivSlash | Div => {
                if float_y == 0.0 {
                    print_error(DivisionByZero);
                    new_el.push(x.to_owned());
                    new_el.push(y.to_owned());
                } else if is_float {
                    new_el.push(Float_(float_x / float_y));
                } else { new_el.push(Int_(int_x / int_y)); }
            }

            // Less than
            LessThan => {
                new_el.push(Bool_(float_x < float_y))
            }

            // Less than or equal to
            LessThanOrEqual => {
                new_el.push(Bool_(float_x <= float_y))
            }

            // Greater than
            GreaterThan => {
                new_el.push(Bool_(float_x > float_y))
            }

            // Greater than or equal to
            GreaterThanOrEqual => {
                new_el.push(Bool_(float_x >= float_y))
            }

            _ => panic!("An error occurred in arithmetic_ops!"),
        };
    } else { print_error(ExpectedNumber) };

    // Return the operator, the original numbers and the new element
    (vec![x, y], new_el)
}


pub fn logical_op(c: Operators, x: bool, y: bool) -> (Vec<Type>, Vec<Type>) {
    let new = match c {
        // Checks whether both predicates are True or not
        And => x && y,

        // Checks whether at least one of the predicates is True or not
        Or => x || y,

        _ => panic!("An error occurred in logical_ops!"),
    };
    
    (vec![Bool_(x), Bool_(y)], vec![Bool_(new)])
}

