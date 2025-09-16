use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedBoolean, ExpectedCodeBlock, ExpectedList, ExpectedListOrString};
use crate::exec;
use crate::stack::Type::{Block_, Bool_, Int_, List_, String_, Variable};
use crate::stack::{Stack, Type};

pub(crate) const CODEBLOCK_OPS: [&str; 4] = [
    "exec",
    "map",
    "each",
    "if",
];
pub(crate) const LIST_OPS: [&str; 5] = ["head", "tail", "empty", "cons", "append"];

pub(crate) fn codeblock(c: String, list: Type, block: Type) -> (Vec<Type>, Vec<Type>) {
    codeblock_custom(c, block, None, Stack::new(), None, Some(list))
}

pub(crate) fn codeblock_custom(c: String, block: Type, then_block: Option<Type>, additional_elems: Stack<Type>, list: Option<Type>, condition: Option<Type>) -> (Vec<Type>, Vec<Type>) {

    let mut new_el = vec![];
    let mut exec_stack = additional_elems.clone();
    
    match c.as_str() {

        // Counts the number of variables in the code block
        "length" => {
            let mut count = 0;
            exec_custom(&mut Some(block.clone()), &mut |_| { count += 1 });
            
            new_el.push(Int_(count));
        }

        // Executes the stack
        "exec" => {

            // Execute the code block for each element in the list
            if let Some(elems) = list.to_owned() {
                exec_stack.push(elems.to_owned());
                exec_stack = exec(Some(block.to_owned()), exec_stack);
            }
            new_el = exec_stack.elements;
        },

        // Execute the regular block if the condition is true
        // Execute the then block if the condition is false
        "if" => {
            if let Some(Bool_(cond)) = condition {
                if let Some(then) = then_block.to_owned() {
                    if cond { exec_stack = exec(Some(then.to_owned()), exec_stack); } else { exec_stack = exec(Some(block.to_owned()), exec_stack); }
                    new_el = exec_stack.elements;
                } else { print_error(ExpectedCodeBlock); return (vec![], vec![]); }
            } else { print_error(ExpectedBoolean); return (vec![], vec![]); }
        },

        "each" | "map" => {

            // Execute the code block for each element in the list
            if let Some(List_(elems)) = list.to_owned() {
                for i in &elems {
                    exec_stack.push(i.to_owned());
                    exec_stack = exec(Some(block.to_owned()), exec_stack);
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
    (vec![Variable(c), list.unwrap_or_default(), block, then_block.unwrap_or_default(), condition.unwrap_or_default()], new_el)
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
fn exec_custom<F>(code_block: &mut Option<Type>, stack_ref: &mut F) -> ()
where F: FnMut(Type) {

    loop {
        // Execute the code from the first element
        match pop_front(code_block) {
            Some(x) => {
                stack_ref(x);
            }

            // Loop through until the list is empty
            _ => break,
        }
    }
}

pub fn pop_front(t: &mut Option<Type>) -> Option<Type> {
    match t {
        Some(Block_(val) | List_(val)) => {
            val.reverse();
            let el = val.pop();
            val.reverse();
            el
        }
        _ => { let el = t.take(); el }
    }
}

pub(crate) fn find_block_elements(stack: Stack<Type>) -> (Stack<Type>, Option<Type>, Option<Type>, Option<Type>, Option<Type>) {
    // Loops through and finds the next operator and list
    let mut list = None;
    let mut additional_elems = Stack::new();

    let mut stack_copy = stack.clone();
    
    // Pop the else block
    stack_copy.pop();
    
    // Stores the then block, the operator, and the condition
    let then_block = stack_copy.pop();
    let operator = stack_copy.pop();
    let condition = stack_copy.pop();

    // Save other elements of the stack
    for elem in stack_copy.elements.iter().rev() {

        if elem.is_list() && list.is_none() {
            list = Some(elem.to_owned());
        }
        else { additional_elems.push(elem.to_owned()); }
        
    }
    
    (additional_elems, list, operator, condition, then_block)
}


