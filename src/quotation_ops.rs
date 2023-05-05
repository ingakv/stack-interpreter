
use crate::error_handling::Error::{ExpectedQuotation};
use crate::error_handling::{print_error};
use crate::list_ops::find_list;
use crate::mylib::{check_operator, is_block, pop_front, string_to_type};
use crate::structs::{Stack, Type};
use crate::structs::Type::{ Int_, List_, String_};

pub(crate) const QUOTATION_OPS: [&str; 6] = [
    "exec",
    "times",
    "map",
    "foldl",
    "each",
    "if",
];


pub(crate) fn do_quotation(pos: i128, stack: Stack<Type>) -> Stack<Type> {

    let mut old_stack = stack.clone();
    old_stack.reverse();

    let mut quotation_op = String_("".to_string());
    let mut count = 0;
    let mut new_stack = vec![];

    // Reorders the stack so that the operators are after the code block
    loop {
        match old_stack.pop() {
            Some(el) => {

                // Switch the position of the operator and the code block in the stack
                if count == pos { quotation_op = el; }

                else if count == pos+1 {
                    new_stack.push(el);
                    new_stack.push(quotation_op.to_owned());
                }

                else { new_stack.push(el); }

                count = count + 1;

            }
            None => {break}
        }
    }


    let mut new = Stack{ elements: (new_stack)};

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

        // Loops through and finds the next code block and list
        let block = find_block(stack, og);
        let mut list = List_(vec![]);

        let mut li_stack = og.clone();

        loop{
            if let Some(li) = li_stack.pop() {
                match li {
                    List_(_) => { list = li; break}
                    _ => {}
                }
            }
            else { break }
        }

        if let Some(x) = block.first() {
            quotation(stack, op, x, list)
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

            let mut new_stack = stack.clone();

            if let List_(elems) = list.to_owned() {

                let list_copy = elems.clone();

                for i in &list_copy {

                    // Execute the code block
                    if let Some(item) = exec(Stack{elements: vec![i.to_owned()]}, block.to_owned()).pop() {
                        new_stack.push(item);
                    }

                }
            }
            new_stack.remove_last_match(list.to_owned());
            new_stack.to_owned()
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
                stack.push(string_to_type(op));

                stack = check_operator((stack.len() - 1) as i128, op, &mut stack.to_owned()).0;
            }

            _ => {break}
        }
    }

    stack

}
