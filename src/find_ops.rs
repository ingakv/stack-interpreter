use crate::combination_ops::COMBINATION_OPS;
use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedBoolean, ExpectedCodeBlock, ExpectedList, ExpectedNumber};
use crate::find_ops::Operations::{Arithmetic, Block, List, Logical};
use crate::list_codeblock_ops::{codeblock, list_op, CODEBLOCK_OPS, LIST_OPS};
use crate::logical_ops::{arithmetic, ARITHMETIC_OPS};
use crate::logical_ops::{logical_op, LOGICAL_OPS};
use crate::stack::Type::{Bool_, List_};
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
    let new_stack = handle_literal_and_operator_recursive(ops, stack, false);
    new_stack.to_owned()
}

pub(crate) fn handle_literal_and_operator_recursive(
    ops: Operations,
    stack: &mut Stack<Type>,
    skip: bool,
) -> Stack<Type> {
    
    let mut old_stack = stack.clone();
    
    // Remove the top element and store it
    let c = stack.pop().unwrap_or_default();

    let st = c.type_to_string_trimmed();

    // Skips if the stack is empty
    if c.is_empty() { Stack::new() }
        
    // Checks if it is an operator
    else if (operation_type(ops).contains(&st.as_str()) || c.is_block()) && !skip {
        // Loops through and finds the next two items of the correct literal type
        let item2 = if !ops.is_block() { handle_literal_and_operator_recursive(ops, stack, true)} else { stack.to_owned() };
        let item1 = if !ops.is_block() { handle_literal_and_operator_recursive(ops, stack, true)} else { stack.to_owned() };

        let mut new_li = old_stack.clone();
        
        // Lists are handled differently
        if let Some(List_(item2_some)) = item2.last() {

            // Loops through and finds the next string
            new_li.elements.pop();
            let str = find_string(&mut new_li);

            // Functions with two lists
            if let Some(_) = item1.last() { list_op(&mut old_stack, st, List_(item2_some), item1.last()) }
                
            // Functions with a list and a string, or only one list
            else { list_op(&mut old_stack, st, List_(item2_some), str) }
        }
        
        else if let Some(item2_some) = item2.last() {

            // Code blocks are handled differently
            if ops.is_block() {
                // Loops through and finds the next operator and list
                let mut list = None;
                let mut op = None;

                loop {
                    if let Some(elem) = new_li.pop() {
                        let elem_str = elem.type_to_string_trimmed();
                        if elem.is_list() && list.is_none() {
                            list = Some(elem)
                        } else if CODEBLOCK_OPS.contains(&elem_str.as_str()) && op.is_none() {
                            op = Some(elem_str)
                        }
                    } else { break; }
                }

                if let (Some(list_some), Some(op_some)) = (list, op) {
                    old_stack = find_wanted_literal_type(ops, &mut old_stack, op_some, list_some, c);
                }

            }
            
            else if let Some(item1_some) = item1.last() {
                old_stack = find_wanted_literal_type(ops, &mut old_stack, st, item1_some, item2_some);
            }

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
        handle_literal_and_operator_recursive(ops, stack, true)
    }
}

fn print_error_literal(wanted_type: Operations) -> () {
    let err = match wanted_type { 
        Arithmetic => ExpectedNumber,
        Logical => ExpectedBoolean,
        List => ExpectedList,
        Block => ExpectedCodeBlock,
    };
    print_error(err)
}
fn is_wanted_literal_type(wanted_type: Operations, elem: Type) -> bool {
    match wanted_type { 
        Arithmetic => elem.is_number() || elem.is_bool(),
        Logical => elem.is_bool(),
        List => elem.is_list(),
        Block => elem.is_block(),
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

fn find_wanted_literal_type(wanted_type: Operations, stack: &mut Stack<Type>, op: String, x: Type, y: Type) -> Stack<Type> {
    match wanted_type { 
        Arithmetic => arithmetic(stack, op, x, y),
        Logical => {
            if let (Bool_(a), Bool_(b)) = (x, y) {
                logical_op(stack, op, a, b)
            } else {
                // Fallback: types were not booleans; return stack unchanged
                stack.to_owned()
            }
        },
        Block => {codeblock(&mut stack.to_owned(), op, x, y)}
        _ => stack.to_owned()
    }
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