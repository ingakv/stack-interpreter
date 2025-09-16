use crate::combination_ops::COMBINATION_OPS;
use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedBoolean, ExpectedList, ExpectedNumber};
use crate::find_ops::Operations::{Arithmetic, Block, List, Logical};
use crate::list_codeblock_ops::{codeblock_custom, find_block_elements, list_op, CODEBLOCK_OPS, LIST_OPS};
use crate::logical_ops::{arithmetic, ARITHMETIC_OPS};
use crate::logical_ops::{logical_op, LOGICAL_OPS};
use crate::stack::Type::{Bool_, List_, Variable};
use crate::stack::{is_string_number, Stack, Type};
use crate::string_ops::{IO_OPS, STACK_OPS, STRING_OPS};

#[derive(Clone, Copy)]
pub enum Operations {
    Arithmetic,
    Logical,
    List,
    Block
}

impl Operations {
    fn is_block(&self) -> bool {
        match self { 
            Block => true,
            _ => false
        }
    }
}

pub(crate) fn handle_literal_and_operator(
    ops: Operations,
    stack: &mut Stack<Type>,
) -> Stack<Type> {
    let new_stack = handle_literal_and_operator_recursive(ops, stack, false, false);
    new_stack.to_owned()
}

pub(crate) fn handle_literal_and_operator_recursive(
    ops: Operations,
    stack: &mut Stack<Type>,
    skip: bool,
    is_if_block: bool,
) -> Stack<Type> {
    
    let mut old_stack = stack.clone();
    
    // Remove the top element and store it
    let c = stack.pop().unwrap_or_default();

    let st = c.type_to_string_trimmed();

    // Skips if the stack is empty
    if c.is_empty() { Stack::new() }
        
    // Checks if it is an operator
    else if (operation_type(ops).contains(&st.as_str()) || c.is_block() || is_if_block) && !skip {
        let mut item2 = stack.to_owned();
        let mut item1 = stack.to_owned();

        // Loops through and finds the next two items of the correct literal type
        if !ops.is_block() {
            item2 = handle_literal_and_operator_recursive(ops, stack, true, false);
            item1 = handle_literal_and_operator_recursive(ops, stack, true, false);
        }
        
        let mut new_li = old_stack.clone();
        
        // Lists are handled differently
        if let Some(List_(item2_some)) = item2.last() {

            // Loops through and finds the next string
            new_li.elements.pop();
            let str = find_string(&mut new_li);

            // Functions with two lists
            let (remove_vec, new_vec) = if let Some(_) = item1.last() { list_op(st, List_(item2_some), item1.last()) }
                
            // Functions with a list and a string, or only one list
            else { list_op(st, List_(item2_some), str) };
            
            old_stack.replace_last_match(remove_vec, new_vec)
        }
        
        else if let Some(item2_some) = item2.last() {

            // Code blocks are handled differently
            if ops.is_block() {
                
                // Finds the next operator and list
                let (additional_elems, list, operator, condition, then_block) = 
                    find_block_elements(old_stack.to_owned());
                
                
                if let Some(Variable(op_some)) = operator {
                    let (rem, new) = codeblock_custom(op_some, c, then_block, additional_elems, list, condition);

                    // Removes the operator, the original numbers or replaces them with the new element
                    old_stack.replace_last_match(rem, new);
                }

            }
            
            else if item1.last().is_some() {
                old_stack = find_wanted_literal_type(ops, &mut old_stack, st, item1.last(), item2_some);
            }

            // Return the stack
            old_stack.to_owned()
        }
            
        // If there are less than two valid items in the stack, the original stack gets sent back
        else {
            print_error_literal(ops);
            old_stack.to_owned()
        }
    } else if is_wanted_literal_type(ops, c.to_owned()) {
        Stack { elements: vec![c] }
    } else {
        handle_literal_and_operator_recursive(ops, stack, true, false)
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
    }
}
fn operation_type(op: Operations) -> &'static [&'static str] {
    match op {
        Arithmetic => &ARITHMETIC_OPS,
        Logical => &LOGICAL_OPS,
        List => &LIST_OPS,
        Block => &CODEBLOCK_OPS,
    }
}

fn find_wanted_literal_type(wanted_type: Operations, stack: &mut Stack<Type>, op: String, x: Option<Type>, y: Type) -> Stack<Type> {
    
    let (remove_vec, new_el) = match wanted_type { 
        Arithmetic => {
            if let Some(a) = x {
                arithmetic(op, a, y)
            } else {
                // Fallback: types were not numbers; return stack unchanged
                print_error(ExpectedNumber);
                return stack.to_owned()
            }
        },
        Logical => {
            if let (Some(Bool_(a)), Bool_(b)) = (x, y) {
                logical_op(op, a, b)
            } else {
                // Fallback: types were not booleans; return stack unchanged
                print_error(ExpectedBoolean);
                return stack.to_owned()
            }
        },
        _ => return stack.to_owned()
    };

    // Removes the operator, the original numbers or replaces them with the new element
    stack.replace_last_match(remove_vec, new_el);

    // Return the stack
    stack.to_owned()

}


pub(crate) fn find_string(stack: &mut Stack<Type>) -> Option<Type> {
    // Remove the top element and store it
    let c = stack.pop().unwrap_or_default();

    // Skips if the stack is empty
    if c.is_empty() { None } 
    else if !is_op(c.type_to_string_trimmed().as_str()) && 
            (c.is_string() || is_string_number(c.type_to_string_trimmed().as_str()))
    { Some(c) }
    else { find_string(stack) }
}


pub(crate) fn is_op(el: &str) -> bool {
    CODEBLOCK_OPS.contains(&el) ||
        IO_OPS.contains(&el) ||
        STACK_OPS.contains(&el) ||
        STRING_OPS.contains(&el) ||
        ARITHMETIC_OPS.contains(&el) ||
        LOGICAL_OPS.contains(&el) ||
        LIST_OPS.contains(&el) ||
        COMBINATION_OPS.contains(&el)
}