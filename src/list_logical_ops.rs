use crate::error_handling::print_error;
use crate::error_handling::Error::{DivisionByZero, ExpectedList, ExpectedListOrString};
use crate::stack::Type::{Bool_, Float_, Int_, List_, String_};
use crate::stack::{Stack, Type};

pub(crate) const ARITHMETIC_OPS: [&str; 7] = ["+", "-", "*", "/", "div", "<", ">"];
pub(crate) const LOGICAL_OPS: [&str; 2] = ["&&", "||"];
pub(crate) const LIST_OPS: [&str; 5] = ["head", "tail", "empty", "cons", "append"];


pub fn arithmetic(stack: &mut Stack<Type>, c: &str, x: Type, y: Type) -> Stack<Type> {
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
    stack.replace_last_match(vec![x, y], new_el);

    stack.to_owned()
}


pub fn logical_op(stack: &mut Stack<Type>, c: &str, x: bool, y: bool) -> Stack<Type> {
    let new = match c {
        // Checks whether both predicates are True or not
        "&&" => x && y,

        // Checks whether at least one of the predicates is True or not
        "||" => x || y,

        _ => panic!("An error occurred in logical_ops!"),
    };

    // Removes the operator and adds the new variable
    stack.pop();

    stack.replace_last_match(vec![Bool_(x), Bool_(y)], Bool_(new));

    // Return the stack
    stack.to_owned()
}


pub(crate) fn list_op(stack: &mut Stack<Type>, c: &str, li: Vec<Type>, el: Type) -> Stack<Type> {

    // Removes the operator
    stack.pop();

    let (new_el, remove_el) = match c {
        // Returns the first item of the list
        "head" => {
            let head = if !li.is_empty() {
                li.first().unwrap().to_owned()
            } else {
                print_error(ExpectedList);
                String_(String::new())
            };
            (head, true)
        }

        // Returns the last item of the list
        "tail" => {
            let mut new_li = li.clone();
            new_li.remove(0);
            (List_(new_li), false)
        }

        // Returns whether the list is empty
        "empty" => (Bool_(li.is_empty()), false),

        // Returns the length of the list
        "length" => (Int_(li.len() as i128), false),

        // Inserts the string onto the front of the list
        "append" => {

            if el.is_empty() {
                print_error(ExpectedListOrString);
            }

            let mut list = vec![el.to_owned()];

            for i in li.to_owned() {
                list.push(i);
            }

            (List_(list), true)
        }

        // Combines the two lists
        "cons" => {

            // Return the other list if one of them is empty
            match el.to_owned() {
                List_(mut i) => {
                    for elem in li.to_owned() {
                        if !i.contains(&elem) { i.push(elem); }
                    }
                    (List_(i), true)
                },
                _ => { print_error(ExpectedList); (List_(vec![el.to_owned()]), true) },
            }
        }

        _ => panic!("An error occurred in list_ops!"),
    };

    // Remove the original numbers or replaces them with the new element
    let mut remove_vec = vec![List_(li)];
    if remove_el {remove_vec.push(el)}

    stack.replace_last_match(remove_vec, new_el);

    // Return the stack
    stack.to_owned()
}