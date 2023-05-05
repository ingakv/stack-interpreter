
use crate::error_handling::Error::{ExpectedQuotation, ExpectedVariable};
use crate::error_handling::{print_error};
use crate::mylib::{check_operator, is_block, pop_front};
use crate::structs::{Stack, Type};
use crate::structs::Type::{Bool_, Float_, Int_, String_};

pub(crate) const QUOTATION_OPS: [&str; 6] = [
    "exec",
    "times",
    "map",
    "foldl",
    "each",
    "if",
];


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

        if let Some(x) = block.first() {
            quotation(stack, op, x)
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



pub(crate) fn quotation(stack: &mut Stack<Type>, c: &str, block: Type) -> Stack<Type> {


    let code = match c {

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
            stack.push(Int_(count)); return stack.to_owned()
        }

        // Executes the stack
        "exec" => { exec(block) },

        // Checks whether at least one of the predicates are True or not
        "times" => { stack.to_owned() },

        // Inverts the predicate
        "map" => { stack.to_owned() },

        // Inverts the predicate
        "foldl" => { stack.to_owned() },

        // Checks whether at least one of the predicates are True or not
        "each" => { stack.to_owned() },

        // Checks whether at least one of the predicates are True or not
        "if" => { stack.to_owned() },

        _ => panic!("An error occurred in quotation_ops!"),
    };


    for i in code.elements {
        match i {
            Int_(i) => stack.push(Int_(i.to_owned())),
            Float_(i) => stack.push(Float_(i.to_owned())),
            Bool_(i) => stack.push(Bool_(i.to_owned())),
            String_(i) => stack.push(String_(i.to_owned())),
//            List_(_) => {}
//            Block_(_) => {}
            _ => { print_error(ExpectedVariable) }

        };
    }

    stack.to_owned()

}




pub(crate) fn exec(block: Type) -> Stack<Type> {

    let mut code = Stack::new();

    let mut old_block = block.clone();


    loop {
        match pop_front(old_block.to_owned()) {

            (Some(x), rem) => {
                old_block = rem;

                let st = x.type_to_string();
                let op = st.trim_start_matches("\"").trim_end_matches("\"");
                code = check_operator(op, &mut code.to_owned());
            }

            _ => {break}
        }
    }


    code

}
