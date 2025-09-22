use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedBoolean, ExpectedCodeBlock, ExpectedList, ExpectedListOrString, ExpectedNumber, ExpectedNumberStringOrList};
use crate::exec;
use crate::stack::Operators::{Append, Cons, Each, Empty, Exec, Foldl, Foldr, Head, If, Length, Loop, Map, Tail, Times};
use crate::stack::Type::{Block_, Bool_, Int_, List_, Variable};
use crate::stack::{Operators, Stack, Type};

pub(crate) fn codeblock_ops(input: String) -> Option<Operators> {
    let res = match input.as_str() { 
        "exec" => {Exec},
        "map" => {Map},
        "each" => {Each},
        "if" => {If},
        "times" => {Times},
        "loop" => {Loop},
        "foldl" => {Foldl},
        "foldr" => {Foldr},
        _ => {return None;}
    };
    Some(res)
}

pub(crate) fn list_ops(input: String) -> Option<Operators> {
    let res = match input.as_str() { 
        "head" => {Head},
        "tail" => {Tail},
        "empty" => {Empty},
        "cons" => {Cons},
        "append" => {Append},
        _ => {return None;}
    };
    Some(res)
}


pub(crate) fn codeblock(c: Operators, list: Type, block: Type) -> (Vec<Type>, Vec<Type>) {
    codeblock_custom(c, Stack::new(), block, None, Some(list))
}

pub(crate) fn codeblock_custom(c: Operators, stack: Stack<Type>, block: Type, block_or_int: Option<Type>, bool_or_list: Option<Type>) -> (Vec<Type>, Vec<Type>) {

    let mut new_el = vec![];
    let mut exec_stack = stack.clone();
    
    match c {

        // Counts the number of variables in the code block
        Length => {
            let count =block.length() as i128;
            
            new_el.push(Int_(count));
        }

        // Executes the stack
        Exec => {

            // Execute the code block for each element in the list
            if let Some(List_(elems)) = bool_or_list.to_owned() {
                exec_stack.push(List_(elems).to_owned());
            }
            exec_stack = exec(block_or_int.to_owned(), exec_stack);
            new_el = exec_stack.elements;
        },

        // Execute the regular block if the condition is true
        // Execute the then block if the condition is false
        If => {
            if let Some(Bool_(cond)) = bool_or_list {
                if let Some(_) = block_or_int.to_owned() {
                    if cond { exec_stack = exec(block_or_int.to_owned(), exec_stack); }
                    else { exec_stack = exec(Some(block.to_owned()), exec_stack); }
                    new_el = exec_stack.elements;
                } else { print_error(ExpectedCodeBlock); return (vec![], vec![]); }
            } else { print_error(ExpectedBoolean); return (vec![], vec![]); }
        },

        Each | Map => {

            // Execute the code block for each element in the list
            if let Some(List_(elems)) = bool_or_list.to_owned() {
                for i in &elems {
                    exec_stack.push(i.to_owned());
                    exec_stack = exec(Some(block.to_owned()), exec_stack);
                }
            } else { print_error(ExpectedList); }
            
            // If the operator is "each", return as a vector of elements,
            // if it is "map" return as a list
            if c == Each { new_el = exec_stack.elements; }
            else { new_el.push(List_(exec_stack.elements)); }
            
        }

        // Execute the code block a given number of times
        Times => {

            if let Some(Int_(num)) = block_or_int.to_owned() {
                for _ in 0..num { exec_stack = exec(Some(block.to_owned()), exec_stack); }
            } else { print_error(ExpectedNumber); }
            new_el = exec_stack.elements;
            
        }
        
        Loop => {
            
            if let Some(Block_(break_block)) = block_or_int.to_owned() {
                loop {
                    // Execute the break block first, which will add True or False to the stack
                    exec_stack = exec(Some(Block_(break_block.to_owned())), exec_stack);

                    // If the top element is True, break the loop
                    if let Some(Bool_(condition)) = exec_stack.pop() {
                        if condition {break; }
                            
                        // If the top element is False, execute the code block
                        else { exec_stack = exec(Some(block.to_owned()), exec_stack); }
                    } else { print_error(ExpectedBoolean); }
                }
                
            } else { print_error(ExpectedCodeBlock); }
            new_el = exec_stack.elements;
            
        }

        Foldl | Foldr => {
            // Check if the second argument is a list
            if let Some(List_(mut list)) = bool_or_list.clone() {
                
                // Reverse the list from left to right if the operator is "foldl"
                if c == Foldl { list.reverse() }
                
                // Check if the first argument is an integer and push it onto the stack
                if let Some(Int_(number)) = block_or_int.to_owned() {
                    exec_stack.push(Int_(number));

                    // Iterate over each element in the list
                    while let Some(list_elem) =list.pop() {
                        exec_stack.push(list_elem);

                        // Execute the code block with the current stack
                        exec_stack = exec(Some(block.to_owned()), exec_stack);
                    }
                } else { print_error(ExpectedNumber); }
            } else { print_error(ExpectedList); }
            
            new_el = exec_stack.elements;
        }

        _ => panic!("An error occurred in codeblocks_ops!"),
    };

    // Return the elements to be removed and the new ones
    (vec![block, block_or_int.unwrap_or_default(), bool_or_list.unwrap_or_default()], new_el)
}

pub(crate) fn list(c: Operators, list: Type, el: Option<Type>) -> (Vec<Type>, Vec<Type>) {

    let mut new_el = vec![];
    let mut remove_vec = vec![];
    if let List_(elems) = list {
        
    match c {
        
        // Returns the first item of the list
        Head => {
            if let Some(head) = elems.first() {
                new_el.push(head.to_owned());
            } else { print_error(ExpectedList) }
        }

        // Returns the last item of the list
        Tail => {
            let mut new_li = elems.clone();
            new_li.remove(0);
            new_el.push(List_(new_li));
        }

        // Returns whether the list is empty
        Empty => new_el.push(Bool_(elems.is_empty())),

        // Returns the length of the list
        Length => new_el.push(Int_(elems.len() as i128)),

        // Inserts the string onto the front of the list
        Append => {

            if let Some(some_el) = el {
                let mut list = vec![some_el.to_owned()];

                for i in elems.to_owned() { list.push(i); }

                new_el.push(List_(list));
                remove_vec.push(some_el);
                
            } else { print_error(ExpectedListOrString); }
        }

        // Combines the two lists
        Cons => {

            if let Some(list) =  el.to_owned() {
                if let List_(mut i) = list.to_owned() {
                    for elem in elems.to_owned() {
                        if !i.contains(&elem) { i.push(elem); }
                    }
                    new_el.push(List_(i));
                    
                // Return the other list if one of them is empty
                } else { new_el.push(List_(vec![list.to_owned()])); }
                remove_vec.push(list);
            } else { print_error(ExpectedNumberStringOrList) }
        }

        _ => panic!("An error occurred in list_ops!"),
    };

    remove_vec.push(List_(elems));
    }

    // Return the operator, the original numbers and the new element
    (remove_vec, new_el)
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

pub(crate) fn find_block_elements(stack: &mut Stack<Type>, code_block: Type, operator: Operators) -> (Stack<Type>, Option<Type>, Option<Type>) {
    
    // Loops through and finds the next operator and list
    let mut block_or_number = None;
    let mut bool_or_list = None;
    let mut additional_elems = Stack::new();


    // Save other elements of the stack
    while let Some(elem) = stack.pop() {

        // Stores the then block, the operator, and the condition
        if [If, Loop].contains(&operator) {
            block_or_number = Some(elem);
            stack.pop(); // Pop the operator
            if operator == If { bool_or_list = stack.pop(); }
        } else if (elem.is_bool() || elem.is_list()) && bool_or_list.is_none() {
            bool_or_list = Some(elem);
        } else if (elem.is_block() || elem.is_number()) && block_or_number.is_none() {
            block_or_number = Some(elem);
        } else if ![code_block.to_owned(), Variable(operator)].contains(&elem) { additional_elems.push_front(elem.to_owned()); }


        let is_bool_or_list = bool_or_list.to_owned().unwrap_or_default();
        let is_block_or_number = block_or_number.to_owned().unwrap_or_default();

        // Ensures that the required elements are present
        if match operator {
            If => { block_or_number.is_some() && is_bool_or_list.is_bool() }
            Loop => { block_or_number.is_some() }
            Map | Each => { is_bool_or_list.is_list() }
            Exec => { is_block_or_number.is_block() }
            Times => { is_block_or_number.is_number() }
            Foldl | Foldr => { is_bool_or_list.is_list() && is_block_or_number.is_number() }
            _ => { true }
        } {
            while let Some(elem) = stack.pop() { additional_elems.push_front(elem); }
            break;
        }
    }

    (additional_elems, bool_or_list, block_or_number)
}


