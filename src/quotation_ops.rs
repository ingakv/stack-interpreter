use crate::error_handling::print_error;
use crate::error_handling::Error::ExpectedQuotation;
use crate::mylib::{check_operator, is_block, is_list, is_op, pop_front};
use crate::structs::Type::{Block_, Int_, List_, String_};
use crate::structs::{Stack, Type};

pub(crate) const QUOTATION_OPS: [&str; 2] = [
    "exec", //    "times",
    //    "map",
    //    "foldl",
    "each",
    //    "if",
];

pub(crate) fn do_quotation(stack: Stack<Type>) -> Stack<Type> {
    // Loops through and finds the next code block and list
    let mut block = Block_(vec![]);
    let mut list = List_(vec![]);
    let mut op = String::new();

    let mut li_stack = stack.clone();

    loop {
        if let Some(li) = li_stack.pop() {
            if is_block(vec![li.to_owned()]) {
                block = li
            } else if is_list(vec![li.to_owned()]) {
                list = li
            } else if is_op(li.type_to_string().as_str()) {
                op = li.type_to_string()
            }
        } else {
            break;
        }
    }

    quotation(&mut stack.to_owned(), op.as_str(), block, list)
}

pub(crate) fn find_block(stack: &mut Stack<Type>) -> Stack<Type> {
    // Remove the top element and store it
    let c = stack.last().unwrap_or_else(|| String_(String::new()));

    // Skips if the stack is empty
    if c.is_empty() {
        print_error(ExpectedQuotation);
        Stack { elements: vec![] }
    } else if is_block(vec![c.to_owned()]) {
        stack.to_owned()
    } else {
        stack.pop();
        find_block(stack)
    }
}

pub(crate) fn quotation(stack: &mut Stack<Type>, c: &str, block: Type, list: Type) -> Stack<Type> {
    let new_stack = match c {
        // Counts the number of variables in the code block
        "length" => {
            let mut copy = block.clone();
            let mut count = 0;

            loop {
                match pop_front(copy.to_owned()) {
                    (Some(_), rem) => {
                        count = count + 1;
                        copy = rem;
                    }
                    _ => break,
                }
            }
            stack.push(Int_(count));
            stack.to_owned()
        }

        // Executes the stack
        "exec" => exec(stack.to_owned(), block),

        // Checks whether at least one of the predicates is True or not
        "times" => stack.to_owned(),

        // Inverts the predicate
        "map" => stack.to_owned(),

        // Inverts the predicate
        "foldl" => stack.to_owned(),

        // Checks whether at least one of the predicates is True or not
        "each" => {
            let mut new_stack = Stack::new();

            if let List_(elems) = list.to_owned() {
                let list_copy = elems.clone();

                for i in &list_copy {
                    // Execute the code block
                    if let Some(item) = exec(
                        Stack {
                            elements: vec![i.to_owned()],
                        },
                        block.to_owned(),
                    )
                    .pop()
                    {
                        new_stack.push(item);
                    }
                }
            }
            new_stack.to_owned()
        }

        // Checks whether at least one of the predicates is True or not
        "if" => stack.to_owned(),

        _ => stack.to_owned(),
    };

    new_stack.to_owned()
}

// Execute a code block
pub(crate) fn exec(mut stack: Stack<Type>, block: Type) -> Stack<Type> {
    let mut old_block = block.clone();

    loop {
        // Execute the code from the first element
        match pop_front(old_block.to_owned()) {
            (Some(x), rem) => {
                old_block = rem;

                let st = x.type_to_string();
                let op = st.trim_start_matches("\"").trim_end_matches("\"");

                stack = check_operator(false, op, &mut stack.to_owned());
            }

            // Loop through until the list is empty
            _ => break,
        }
    }

    stack
}
