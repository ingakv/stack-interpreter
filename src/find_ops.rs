use crate::list_logical_ops::{arithmetic, ARITHMETIC_OPS};
use crate::list_logical_ops::{list_op, LIST_OPS};
use crate::list_logical_ops::{logical_op, LOGICAL_OPS};
use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedBoolean, ExpectedList, ExpectedNumber};
use crate::stack::Type::{Bool_, List_, String_};
use crate::stack::{Stack, Type};
use crate::string_ops::find_string;

#[derive(Clone, Copy)]
pub enum Operations {
    Arithmetic,
    Logical,
    List
}

pub(crate) fn handle_literal_and_operator(
    ops: Operations,
    stack: &mut Stack<Type>,
    og: &mut Stack<Type>,
    skip: bool,
) -> Stack<Type> {
    // Remove the top element and store it
    let c = stack.pop().unwrap_or_else(|| String_(String::new()));

    let s = c.type_to_string();
    let st = s.trim_start_matches("\"").trim_end_matches("\"");

    // Skips if the stack is empty
    if c.is_empty() {
        Stack::new()
    }
        
    // Checks if it is an operator
    else if operation_type(ops).contains(&st) && !skip {
        // Loops through and finds the next two items of the correct literal type
        let item2 = handle_literal_and_operator(ops, stack, og, true);
        let item1 = handle_literal_and_operator(ops, stack, og, true);
        
        // Lists are handled differently
        if let Some(List_(x)) = item2.first() {

            // Loops through and finds the next non-list (AKA string)
            let mut new_li = stack.clone();
            new_li.elements.pop();
            let str = find_string(&mut new_li);

            // Functions with two lists
            if let Some(y) = item1.first() { list_op(og, &st, x, y) }
                
            // Functions with a list and a string
            else if let Some(y) = str.first() { list_op(og, &st, x, y) }

            // Functions that require only one list
            else {
                match st {
                    // This is to return the value to quotation_ops
                    "each" => Stack {
                        elements: x,
                    },
                    _ => list_op(og, &st, x, String_(String::new())),
                }
            }
        }
        
        else if let (Some(x), Some(y)) = (item1.first(), item2.first()) {
            find_wanted_literal_type(ops, og, &st, x, y)
        }
            
        // If there are less than two valid items in the stack, the original stack gets sent back
        // (without the operator)
        else {
            print_error_literal(ops);
            og.pop();
            og.to_owned()
        }
    } else if is_wanted_literal_type(ops, c.to_owned()) {
        Stack { elements: vec![c] }
    } else {
        handle_literal_and_operator(ops, stack, og, true)
    }
}

fn print_error_literal(wanted_type: Operations) -> () {
    let err = match wanted_type { 
        Operations::Arithmetic => ExpectedNumber,
        Operations::Logical => ExpectedBoolean,
        Operations::List => ExpectedList
    };
    print_error(err)
}
fn is_wanted_literal_type(wanted_type: Operations, elem: Type) -> bool {
    match wanted_type { 
        Operations::Arithmetic => elem.is_number(),
        Operations::Logical => elem.is_bool(),
        Operations::List => elem.is_list()
    }
}
fn operation_type(op: Operations) -> &'static [&'static str] {
    match op {
        Operations::Arithmetic => &ARITHMETIC_OPS,
        Operations::Logical => &LOGICAL_OPS,
        Operations::List => &LIST_OPS
    }
}

fn find_wanted_literal_type(wanted_type: Operations, stack: &mut Stack<Type>, op: &str, x: Type, y: Type) -> Stack<Type> {
    match wanted_type { 
        Operations::Arithmetic => arithmetic(stack, op, x, y),
        Operations::Logical => {
            if let (Bool_(a), Bool_(b)) = (x, y) {
                logical_op(stack, op, a, b)
            } else {
                // Fallback: types were not booleans; return stack unchanged
                stack.to_owned()
            }
        }
        _ => stack.to_owned()
    }
}
