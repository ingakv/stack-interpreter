use std::ffi::c_void;
use crate::error_handling::Error::{ExpectedQuotation};
use crate::error_handling::{print_error};
use crate::list_ops::find_list;
use crate::mylib::{check_operator, is_block, pop_front};
use crate::structs::{Stack, Type};
use crate::structs::Type::{Block_, Int_, List_, String_};

pub(crate) const QUOTATION_OPS: [&str; 6] = [
    "exec",
    "times",
    "map",
    "foldl",
    "each",
    "if",
];


pub(crate) fn do_quotation(stack: Stack<Type>) -> Stack<Type> {

    let mut old_stack = stack.clone();
    let mut sorted_stack = vec![];
    let mut quotation_ops = vec![];

    // Reorders the stack so that the operators are at the end
    loop {
        match old_stack.pop() {
            Some(el) => {
                if QUOTATION_OPS.contains(&el.type_to_string().as_str()) { quotation_ops.push(el) }
                else { sorted_stack.push(el) }
            }
            None => {break}
        }
    }

    // Since it gets reversed in the loop above, it needs to be reversed back
    sorted_stack.reverse();

    for i in quotation_ops {
        sorted_stack.push(i);
    }

    let mut new = Stack{ elements: (sorted_stack)};

    let mut new2 = new.clone();

    find_block(&mut new, &mut new2)

}


pub(crate) fn find_block(stack: &mut Stack<Type>, og: &mut Stack<Type>) -> Stack<Type> {

    // Remove top element and store it
    let c = stack.pop().unwrap_or_else(|| String_("".to_string()));

    let st = c.type_to_string();
    let op = st.trim_start_matches("\"").trim_end_matches("\"");

    // Skips if the stack is empty
    if c == String_("".to_string()) {
        Stack{ elements: vec![] }
    }

    // Checks if it is an operator
    else if QUOTATION_OPS.contains(&op) {
        // Loops through and finds the next two numbers
        let block = find_block(stack, og);
        let list = find_list(og, &mut og.clone());

        if let (Some(x), Some(y)) = (block.first(), list.first()) {
            quotation(stack, op, x, y)
        }

        else if let Some(x) = block.first() {
            quotation(stack, op, x, List_(vec![]))
        }

        // If there are no code blocks in the stack, the original stack gets sent back
        // (without the operator)
        else {
            print_error(ExpectedQuotation);
            og.pop();
            og.to_owned()
        }

    }

    else if is_block(vec![c.to_owned()]) {
        Stack{ elements: vec![c] }
    }

    else {
        find_block(stack, og)
    }
}



pub(crate) fn quotation(stack: &mut Stack<Type>, c: &str, block: Type, list: Type) -> Stack<Type> {

    let new_stack = match c {

        // Counts the amount of variables in the code block
        "length" => {

            let mut copy = block.clone();
            let mut count = 0;

            loop {
                match pop_front(copy.to_owned()) {
                    (Some(_), rem) => {
                        count = count + 1;
                        copy = rem;
                    }
                    _ => {break}
                }
            }
            stack.push(Int_(count));
            stack.to_owned()
        }

        // Executes the stack
        "exec" => { exec(stack.to_owned(), block) },

        // Checks whether at least one of the predicates are True or not
        "times" => { stack.to_owned() },

        // Inverts the predicate
        "map" => { stack.to_owned() },

        // Inverts the predicate
        "foldl" => { stack.to_owned() },

        // Checks whether at least one of the predicates are True or not
        "each" => {

            if let List_(elems) = list.to_owned() {

                let mut list_copy = elems.clone();

                loop {

                    // If there are more items left in the list
                    if let Some(elem) = list_copy.pop() {

                        // Push the element to the front of the code block
                        let mut new_block = vec![elem];

                        if let Block_(mut block_elems) = block.to_owned() {
                            if let Some(elem) = block_elems.pop() {
                                new_block.push(elem)
                            }
                        }

                        // Execute the code block
                        exec(stack.to_owned(), Block_(new_block));
                    }
                    else { break }
                }
            }
            stack.to_owned()
        },

        // Checks whether at least one of the predicates are True or not
        "if" => { stack.to_owned() },

        _ => panic!("An error occurred in quotation_ops!"),
    };


    new_stack.to_owned()

}




pub(crate) fn exec(mut stack: Stack<Type>, block: Type) -> Stack<Type> {

    let mut old_block = block.clone();

    loop {
        match pop_front(old_block.to_owned()) {

            (Some(x), rem) => {
                old_block = rem;

                let st = x.type_to_string();
                let op = st.trim_start_matches("\"").trim_end_matches("\"");
                stack = check_operator(true, op, &mut stack.to_owned());
            }

            _ => {break}
        }
    }

    stack

}
