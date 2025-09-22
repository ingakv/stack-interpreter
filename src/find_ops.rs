use crate::combination_ops::combination_ops;
use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedBoolean, ExpectedList, ExpectedNumber};
use crate::find_ops::Operations::{Arithmetic, Block, List, Logical, StackIO, Strings};
use crate::list_codeblock_ops::{codeblock_custom, codeblock_ops, find_block_elements, list, list_ops};
use crate::logical_ops::{arithmetic, arithmetic_ops};
use crate::logical_ops::{logical_op, logical_ops};
use crate::stack::Type::{Bool_, Variable};
use crate::stack::{is_string_number, Operators, Stack, Type};
use crate::string_ops::{stack_io, stack_io_ops, string_ops, strings_ops};

#[derive(Clone, Copy)]
pub enum Operations {
    Arithmetic,
    Logical,
    Strings,
    StackIO,
    List,
    Block
}

impl Operations {
    pub(crate) fn is_block(&self, type_el: Type) -> bool {
        matches!(self, Block) || type_el.is_block() 
    }
    pub(crate) fn is_list(&self, type_el: Option<Type>) -> bool {
        if let Some(type_list) = type_el {
            matches!(self, List) && type_list.is_list()
        } else { false }
    }
}

pub(crate) fn handle_literal_and_operator(
    ops: Operators,
    stack: &mut Stack<Type>,
) -> Stack<Type> {
    let new_stack = handle_literal_and_operator_recursive(ops, stack, false);
    new_stack.to_owned()
}

pub(crate) fn handle_literal_and_operator_recursive(
    op: Operators,
    stack: &mut Stack<Type>,
    skip: bool,
) -> Stack<Type> {
    
    // Remove the top element and store it
    let Some(c) = stack.pop() else { return Stack::new() };

    let old_stack = &mut stack.clone();

    let ops = op.operator_to_type();
    
    let is_block = ops.is_block(c.to_owned());
        
    // Checks if it is an operator
    if (string_to_operator(c.type_to_string_trimmed()).is_some() || is_block) && !skip {
        let mut item2 = old_stack.to_owned();
        let mut item1 = old_stack.to_owned();

        // Loops through and finds the next two items of the correct literal type
        if !is_block {
            item2 = handle_literal_and_operator_recursive(op, old_stack, true);
            item1 = handle_literal_and_operator_recursive(op, old_stack, true);
        }
        
        
        let mut new_li = stack.clone();
        
        // Lists are handled differently
        if ops.is_list(item2.last()) {
            if let Some(item2_some) = item2.last() {
    
                // Loops through and finds the next string
                let str = find_string(&mut new_li);
    
                // Functions with two lists
                let (remove_vec, new_vec) = 
                    if let (Some(_), Variable(st)) = (item1.last(), c.to_owned()) { 
                        list(st, item2_some, item1.last()) 
                    }
                    
                    // Functions with a list and a string, or only one list
                    else if let Variable(st) = c { 
                        list(st, item2_some, str) 
                    } else { (vec![], vec![]) };
                
                stack.replace_last_match(remove_vec, new_vec);
            }
            stack.to_owned()
        }
        
        else if let Some(item2_some) = item2.last() {

            // Code blocks are handled differently
            if is_block {
                
                // Finds the next operator and list
                let (additional_elems, bool_or_list, block_or_number) = 
                    find_block_elements(stack, c.to_owned(), op);
                
                
                let (rem, new) = codeblock_custom(op, additional_elems, c, block_or_number, bool_or_list);

                // Removes the operator, the original numbers or replaces them with the new element
                stack.replace_last_match(rem, new);

            }
            
            else if let Variable(st) = c {
                find_wanted_literal_type(ops, stack, st, item1.last(), item2_some);
            }

            // Return the stack
            stack.to_owned()
        }
            
        // If there are less than two valid items in the stack, the original stack gets sent back
        else {
            print_error_literal(ops);
            stack.to_owned()
        }
    } else if is_wanted_literal_type(ops, c.to_owned()) {
        Stack { elements: vec![c] }
    } else {
        handle_literal_and_operator_recursive(op, old_stack, true)
    }
}

fn print_error_literal(wanted_type: Operations) -> () {
    match wanted_type {
        Arithmetic => print_error(ExpectedNumber),
        Logical => print_error(ExpectedBoolean),
        List => print_error(ExpectedList),
        _ => {}
    };
}
fn is_wanted_literal_type(wanted_type: Operations, elem: Type) -> bool {
    match wanted_type { 
        Arithmetic => elem.is_number() || elem.is_bool(),
        Logical => elem.is_bool(),
        List => elem.is_list(),
        Block => !elem.is_list(),
        _ => true
    }
}

fn find_wanted_literal_type(wanted_type: Operations, stack: &mut Stack<Type>, op: Operators, x: Option<Type>, y: Type) {
    
    let (remove_vec, new_el) = match wanted_type { 
        Arithmetic => {
            if let Some(a) = x {
                arithmetic(op, a, y)
            } else {
                // Fallback: types were not numbers; return stack unchanged
                print_error(ExpectedNumber);
                return
            }
        },
        Logical => {
            if let (Some(Bool_(a)), Bool_(b)) = (x, y) {
                logical_op(op, a, b)
            } else {
                // Fallback: types were not booleans; return stack unchanged
                print_error(ExpectedBoolean);
                return
            }
        },
        Strings => { string_ops(op, stack) },
        StackIO => { stack_io(op, (stack.last(), stack.second_to_last())) },
        _ => return
    };

    // Removes the operator, the original numbers or replaces them with the new element
    stack.replace_last_match(remove_vec, new_el);

}


pub(crate) fn find_string(stack: &mut Stack<Type>) -> Option<Type> {
    // Remove the top element and store it
    let c = stack.pop().unwrap_or_default();

    // Skips if the stack is empty
    if c.is_empty() { None } 
    else if string_to_operator(c.type_to_string_trimmed()).is_none() && 
            (c.is_string() || is_string_number(c.type_to_string_trimmed().as_str()))
    { Some(c) }
    else { find_string(stack) }
}


pub(crate) fn string_to_operator(elem: String) -> Option<Operators> {
    let el = elem.to_lowercase();
    if let Some(ans) = combination_ops(el.to_owned()).or_else(
        || codeblock_ops(el.to_owned())).or_else(
        || stack_io_ops(el.to_owned())).or_else(
        || strings_ops(el.to_owned())).or_else(
        || arithmetic_ops(el.to_owned())).or_else(
        || logical_ops(el.to_owned())).or_else(
        || list_ops(el.to_owned())) { return Some(ans) }
    None
}