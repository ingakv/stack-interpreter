use crate::error_handling::Error::ExpectedVariable;
use crate::error_handling::print_error;
use crate::mylib::{invert_number, is_number};
use crate::structs::{Stack, Type};
use crate::structs::Type::{Bool_, String_};

pub(crate) const LOGICAL_OPS: [&str; 3] = ["&&", "||", "not"];

pub(crate) fn find_logical(stack: &mut Stack<Type>, og: &mut Stack<Type>, skip: bool) -> Stack<Type> {

    // Remove top element and store it
    let c = stack.pop().unwrap_or_else(|| String_("".to_string()));


    let st = c.type_to_string();
    let op = st.trim_start_matches("\"").trim_end_matches("\"");


    // Skips if the stack is empty
    if c == String_("".to_string()) {
        Stack{ elements: vec![] }
    }

    // Checks if it is an operator
    else if LOGICAL_OPS.contains(&op) && !skip {
        // Loops through and finds the next two literals
        let num2 = find_logical(stack, og, true);
        let num1 = find_logical(stack, og, true);

        let number = num2.first().unwrap();

        if let (Some(Bool_(x)), Some(Bool_(y))) = (num1.first(), num2.first()) {
            logical_op(og, &op, x, y)
        }

        // If there is only 1 variable, it gets pushed back on, and the stack returns, unless "not" is used
        else if c == String_("not".to_string()) &&
            (is_number(number.type_to_string().as_str())
          || number.is_bool()) {

            let new_nr =
            if number.is_bool() { if number.type_to_bool() { Bool_(false) } else { Bool_(true) } }
            else { invert_number(number.type_to_string().as_str()) };


            // Removes the operator and adds the new variable
            stack.pop();
            stack.replace_last_match(vec![number.to_owned()], new_nr);
            stack.to_owned()

        }

        // If there are less than two valid numbers in the stack, the original stack gets sent back
        // (without the operator)
        else {
            print_error(ExpectedVariable);
            og.pop();
            og.to_owned()
        }

    }

    else if c.is_bool() {
        Stack{ elements: vec![c] }
    }

    else {
        find_logical(stack, og, true)
    }
}


pub fn logical_op(stack: &mut Stack<Type>, c: &str, x: bool, y: bool) -> Stack<Type> {

    let new = match c {
        // Checks whether both predicates are True or not
        "&&" => { x && y },

        // Checks whether at least one of the predicates are True or not
        "||" => { x || y },


        // Inverts the predicate
        "not" => { !x },

        _ => panic!("An error occurred in logical_ops!"),
    };


    // Removes the operator and adds the new variable
    stack.pop();

    stack.replace_last_match(vec![Bool_(x), Bool_(y)], Bool_(new));

    // Return the stack
    stack.to_owned()
}

