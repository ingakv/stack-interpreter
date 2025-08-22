use crate::error_handling::Error::{ExpectedList, ExpectedListOrString};
use crate::error_handling::print_error;
use crate::mylib::check_operator;
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
    let new_stack = match c.as_str(){
        // Counts the number of variables in the code block
        "length" => {
            let mut count = 0;
            exec_custom(block.clone(), &mut |_| { count += 1 });

            stack.push(Int_(count));
            stack.to_owned()
        }

        // Executes the stack
        "exec" => {
            exec(block.to_owned(), stack.to_owned());
            stack.to_owned()
        },

        // Checks whether at least one of the predicates is True or not
        "each" => {
            let mut new_stack = Stack::new();

            if let List_(elems) = list {
                let list_copy = elems.clone();

                for i in &list_copy {
                    // Execute the code block
                    if let Some(item) = exec(
                        block.to_owned(), 
                        Stack { elements: vec![i.to_owned()], }
                    ).pop() {
                        new_stack.push(item);
                    }
                }
            }
            new_stack
        }

        _ => stack.to_owned(),
    };

    new_stack
}



// Execute a code block
fn exec_custom<F>(mut t: Type, f: &mut F) -> ()
where F: FnMut(Type) {

    loop {
        // Execute the code from the first element
        match pop_front(t.to_owned()) {
            (Some(x), rem) => {
                t = rem;
                f(x);
            }

            // Loop through until the list is empty
            _ => break,
        }
    }
}


// Execute a code block
fn exec(block: Type, mut stack: Stack<Type>) -> Stack<Type> {
    exec_custom(block.clone(), &mut |x| { stack = check_operator(x, &mut stack); });
    stack
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

pub(crate) fn list_op(stack: &mut Stack<Type>, c: String, li: Vec<Type>, el: Type) -> Stack<Type> {

    let (new_el, remove_el) = match c.as_str() {
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

    // Removes the operator, the original numbers or replaces them with the new element
    let mut remove_vec = vec![Variable(c)];
    remove_vec.push(List_(li));
    if remove_el {remove_vec.push(el)}

    stack.replace_last_match(remove_vec, new_el);

    // Return the stack
    stack.to_owned()
}