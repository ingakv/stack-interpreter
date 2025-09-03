use crate::check_operator;
use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedList, ExpectedListOrString};
use crate::stack::Type::{Block_, Bool_, Int_, List_, String_, Variable};
use crate::stack::{Stack, Type};

pub(crate) const CODEBLOCK_OPS: [&str; 3] = [
    "exec", //    "times",
    "map",
    //    "foldl",
    "each",
    //    "if",
];
pub(crate) const LIST_OPS: [&str; 5] = ["head", "tail", "empty", "cons", "append"];

pub(crate) fn codeblock(c: String, list: Type, block: Type) -> (Vec<Type>, Vec<Type>) {
    codeblock_custom(c, Stack::new(), list, block)
}

pub(crate) fn codeblock_custom(c: String, additional_elems: Stack<Type>, list: Type, block: Type) -> (Vec<Type>, Vec<Type>) {

    let mut new_el = vec![];
    let mut exec_stack = additional_elems.clone();
    
    match c.as_str() {

        // Counts the number of variables in the code block
        "length" => {
            let mut count = 0;
            exec_custom(block.clone(), &mut |_| { count += 1 });
            
            new_el.push(Int_(count));
        }

        // Executes the stack
        "exec" => {
            exec_stack.push(list.to_owned());
            exec(block.to_owned(), exec_stack.to_owned());
            new_el.push(list.to_owned());
        },

        "each" | "map" => {

            // Execute the code block for each element in the list
            if let List_(elems) = list.to_owned() {
                for i in &elems {
                    exec_stack.push(i.to_owned());
                    exec_stack = exec(block.to_owned(), exec_stack);
                }
            }
            
            // If the operator is "each", return as a vector of elements,
            // if it is "map" return as a list
            if c.as_str() == "each" { new_el = exec_stack.elements; }
            else { new_el.push(List_(exec_stack.elements)); }
            
        }

        _ => panic!("An error occurred in codeblocks_ops!"),
    };

    // Return the elements to be removed and the new ones
    (vec![Variable(c), list, block], new_el)
}

pub(crate) fn list_op(c: String, list: Type, el: Option<Type>) -> (Vec<Type>, Vec<Type>) {

    let mut new_el = vec![];
    let mut remove_vec = vec![];
    if let List_(elems) = list {
        
    match c.as_str() {
        // Returns the first item of the list
        "head" => {
            let head = elems.first().cloned().unwrap_or_else(|| String_(String::new()));
            new_el.push(head);
            remove_vec.push(el.unwrap_or_default());
        }

        // Returns the last item of the list
        "tail" => {
            let mut new_li = elems.clone();
            new_li.remove(0);
            new_el.push(List_(new_li));
        }

        // Returns whether the list is empty
        "empty" => new_el.push(Bool_(elems.is_empty())),

        // Returns the length of the list
        "length" => new_el.push(Int_(elems.len() as i128)),

        // Inserts the string onto the front of the list
        "append" => {

            if el.is_none() {
                print_error(ExpectedListOrString);
            }
            let mut list = vec![el.to_owned().unwrap_or_default()];

            for i in elems.to_owned() {
                list.push(i);
            }

            new_el.push(List_(list));
            remove_vec.push(el.unwrap_or_default());
        }

        // Combines the two lists
        "cons" => {

            // Return the other list if one of them is empty
            match el.to_owned().unwrap_or_default() {
                List_(mut i) => {
                    for elem in elems.to_owned() {
                        if !i.contains(&elem) { i.push(elem); }
                    }
                    new_el.push(List_(i));
                },
                _ => {
                    print_error(ExpectedList);
                    new_el.push(List_(vec![el.to_owned().unwrap_or_default()]));
                }
            }
            remove_vec.push(el.unwrap_or_default());
        }

        _ => panic!("An error occurred in list_ops!"),
    };

    remove_vec.push(Variable(c.to_owned()));
    remove_vec.push(List_(elems));

    }

    // Return the operator, the original numbers and the new element
    (remove_vec, new_el)
}


// Execute a code block
fn exec(mut block: Type, mut stack: Stack<Type>) -> Stack<Type> {
    
    loop {
        // Execute the code from the first element
        match pop_front(block.to_owned()) {
            (Some(elem), rem) => {
                block = rem;
                stack.push(elem.to_owned());
                stack = check_operator(elem, &mut stack);
            }

            // Loop through until the list is empty
            _ => break,
        }
    }
    stack
}

// Execute a code block
fn exec_custom<F>(mut code_block: Type, stack_ref: &mut F) -> ()
where F: FnMut(Type) {

    loop {
        // Execute the code from the first element
        match pop_front(code_block.to_owned()) {
            (Some(x), rem) => {
                code_block = rem;
                stack_ref(x);
            }

            // Loop through until the list is empty
            _ => break,
        }
    }
}


fn pop_front(t: Type) -> (Option<Type>, Type) {
    match t {
        Block_(val) => {
            let mut new = val.clone();

            new.reverse();
            let el = new.pop();
            new.reverse();

            (el, Block_(new))
        }
        _ => { (None, t) }
    }
}
