use crate::check_operator;
use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedList, ExpectedListOrString};
use crate::stack::Type::{Block_, Bool_, Int_, List_, String_, Variable};
use crate::stack::{Stack, Type};

pub(crate) const CODEBLOCK_OPS: [&str; 2] = [
    "exec", //    "times",
    //    "map",
    //    "foldl",
    "each",
    //    "if",
];
pub(crate) const LIST_OPS: [&str; 5] = ["head", "tail", "empty", "cons", "append"];

pub(crate) fn codeblock(stack: &mut Stack<Type>, c: String, list: Type, block: Type) -> Stack<Type> {

    let mut new_el = vec![];
    match c.as_str() {

        // Counts the number of variables in the code block
        "length" => {
            let mut count = 0;
            exec_custom(block.clone(), &mut |_| { count += 1 });
            
            new_el.push(Int_(count));
        }

        // Executes the stack
        "exec" => {
            exec(block.to_owned(), list.to_owned());
            new_el.push(list.to_owned());
        },

        "each" => {
            let mut new_list = vec![];

            if let List_(elems) = list.to_owned() {
                for i in &elems {
                    // Execute the code block
                    if let Some(item) = exec(block.to_owned(), i.to_owned()) {
                        new_list.push(item);
                    }
                }
            }
            new_el = new_list;
        }

        _ => panic!("An error occurred in list_ops!"),
    };


    // Removes the operator, the original numbers or replaces them with the new element
    let remove_vec = vec![Variable(c), list, block];

    stack.replace_last_match(remove_vec, new_el);

    
    // Return the stack
    stack.to_owned()
}

pub(crate) fn list_op(stack: &mut Stack<Type>, c: String, list: Type, el: Option<Type>) -> Stack<Type> {

    if let List_(elems) = list {
        
    let empty = elems.is_empty();
        
    let (new_el, remove_el) = match c.as_str() {
        // Returns the first item of the list
        "head" => {
            if empty { print_error(ExpectedList);}
            let head = elems.first().cloned().unwrap_or_else(|| String_(String::new()));
            (head, true)
        }

        // Returns the last item of the list
        "tail" => {
            let mut new_li = elems.clone();
            new_li.remove(0);
            (List_(new_li), false)
        }

        // Returns whether the list is empty
        "empty" => (Bool_(empty), false),

        // Returns the length of the list
        "length" => (Int_(elems.len() as i128), false),

        // Inserts the string onto the front of the list
        "append" => {

            if el.is_none() {
                print_error(ExpectedListOrString);
            }
            let mut list = vec![el.to_owned().unwrap_or_default()];

            for i in elems.to_owned() {
                list.push(i);
            }

            (List_(list), true)
        }

        // Combines the two lists
        "cons" => {

            // Return the other list if one of them is empty
            match el.to_owned().unwrap_or_default() {
                List_(mut i) => {
                    for elem in elems.to_owned() {
                        if !i.contains(&elem) { i.push(elem); }
                    }
                    (List_(i), true)
                },
                _ => { print_error(ExpectedList); (List_(vec![el.to_owned().unwrap_or_default()]), true) },
            }
        }

        _ => panic!("An error occurred in list_ops!"),
    };

    // Removes the operator, the original numbers or replaces them with the new element
    let mut remove_vec = vec![Variable(c)];
    remove_vec.push(List_(elems));
    if remove_el {remove_vec.push(el.unwrap_or_default())}

    stack.replace_last_match(remove_vec, vec![new_el]);

    }
    // Return the stack
    stack.to_owned()
}


// Execute a code block
fn exec(mut block: Type, list_elem: Type) -> Option<Type> {

    let mut new_stack = Stack { elements: vec![list_elem.to_owned()], };
    
    loop {
        // Execute the code from the first element
        match pop_front(block.to_owned()) {
            (Some(elem), rem) => {
                block = rem;
                new_stack.push(elem.to_owned());
                new_stack = check_operator(elem, &mut new_stack);
            }

            // Loop through until the list is empty
            _ => break,
        }
    }
    new_stack.pop()
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
