use crate::error_handling::print_error;
use crate::error_handling::Error::ExpectedVariable;
use crate::stack::Type::{Bool_, String_};
use crate::stack::{Stack, Type};

pub(crate) const LOGICAL_OPS: [&str; 2] = ["&&", "||"];

pub(crate) fn find_logical(
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
    else if LOGICAL_OPS.contains(&op) && !skip {
        // Loops through and finds the next two literals
        let num2 = find_logical(stack, og, true);
        let num1 = find_logical(stack, og, true);

        if let (Some(Bool_(x)), Some(Bool_(y))) = (num1.first(), num2.first()) {
            logical_op(og, &op, x, y)
        }
        // If there are less than two valid numbers in the stack, the original stack gets sent back
        // (without the operator)
        else {
            print_error(ExpectedVariable);
            og.pop();
            og.to_owned()
        }
    } else if c.is_bool() {
        Stack { elements: vec![c] }
    } else {
        find_logical(stack, og, true)
    }
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
