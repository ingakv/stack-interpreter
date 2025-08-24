use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedBoolean, ExpectedList, ExpectedNumber, ExpectedCodeBlock};
use crate::logical_ops::{arithmetic, ARITHMETIC_OPS};
use crate::logical_ops::{logical_op, LOGICAL_OPS};
use crate::is_op;
use crate::list_codeblock_ops::{list_op, codeblock, LIST_OPS, CODEBLOCK_OPS};
use crate::stack::Type::{Bool_, List_, String_};
use crate::stack::{is_list, is_string_number, Stack, Type};

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
            Operations::Block => true,
            _ => false
        }
    }
}

pub(crate) fn handle_literal_and_operator(
    ops: Operations,
    stack: &mut Stack<Type>,
    skip: bool,
) -> Stack<Type> {
    
    let mut old_stack = stack.clone();
    
    // Remove the top element and store it
    let c = stack.pop().unwrap_or_else(|| String_(String::new()));

    let st = c.type_to_string_trimmed();

    // Skips if the stack is empty
    if c.is_empty() { Stack::new() }
        
    // Checks if it is an operator
    else if operation_type(ops).contains(&st.as_str()) && !skip {
        // Loops through and finds the next two items of the correct literal type
        let item2 = handle_literal_and_operator(ops, stack, true);
        let item1 = if !ops.is_block() {handle_literal_and_operator(ops, stack, true)} else { stack.to_owned() };

        let mut new_li = old_stack.clone();
        
        // Lists are handled differently
        if let Some(List_(x)) = item2.first() {

            // Loops through and finds the next non-list (AKA string)
            new_li.elements.pop();
            let str = find_string(&mut new_li);

            // Functions with two lists
            if let Some(y) = item1.first() { list_op(&mut old_stack, st, x, y) }
                
            // Functions with a list and a string
            else if let Some(y) = str.first() { list_op(&mut old_stack, st, x, y) }

            // Functions that require only one list
            else {
                match st.as_str() {
                    // This is to return the value to codeblock_ops
                    "each" => Stack {
                        elements: x,
                    },
                    _ => list_op(&mut old_stack, st, x, String_(String::new())),
                }
            }
        }
        
        else if let Some(y) = item2.first() {

            // Code blocks are handled differently
            if ops.is_block() {
                // Loops through and finds the next operator and list
                let mut list = List_(vec![]);
                let mut op = String::new();

                loop {
                    if let Some(elem) = new_li.pop() {
                        if is_list(vec![elem.to_owned()]) && list.is_empty() {
                            list = elem
                        } else if is_op(elem.type_to_string_trimmed().as_str()) && op.is_empty() {
                            op = elem.type_to_string_trimmed()
                        }
                    } else { break; }
                }

            }
            
            if let Some(x) = item1.first() {
                find_wanted_literal_type(ops, &mut old_stack, st, x, y);
            }
            
            old_stack.to_owned()
        }
            
        // If there are less than two valid items in the stack, the original stack gets sent back
        // (without the operator)
        else {
            print_error_literal(ops);
            old_stack.pop();
            old_stack.to_owned()
        }
    } else if is_wanted_literal_type(ops, c.to_owned()) {
        Stack { elements: vec![c] }
    } else {
        handle_literal_and_operator(ops, stack, true)
    }
}

fn print_error_literal(wanted_type: Operations) -> () {
    let err = match wanted_type { 
        Operations::Arithmetic => ExpectedNumber,
        Operations::Logical => ExpectedBoolean,
        Operations::List => ExpectedList,
        Operations::Block => ExpectedCodeBlock
    };
    print_error(err)
}
fn is_wanted_literal_type(wanted_type: Operations, elem: Type) -> bool {
    match wanted_type { 
        Operations::Arithmetic => elem.is_number() || elem.is_bool(),
        Operations::Logical => elem.is_bool(),
        Operations::List => elem.is_list(),
        Operations::Block => elem.is_block()
    }
}
fn operation_type(op: Operations) -> &'static [&'static str] {
    match op {
        Operations::Arithmetic => &ARITHMETIC_OPS,
        Operations::Logical => &LOGICAL_OPS,
        Operations::List => &LIST_OPS,
        Operations::Block => &CODEBLOCK_OPS
    }
}

fn find_wanted_literal_type(wanted_type: Operations, stack: &mut Stack<Type>, op: String, x: Type, y: Type) -> Stack<Type> {
    match wanted_type { 
        Operations::Arithmetic => arithmetic(stack, op, x, y),
        Operations::Logical => {
            if let (Bool_(a), Bool_(b)) = (x, y) {
                logical_op(stack, op, a, b)
            } else {
                // Fallback: types were not booleans; return stack unchanged
                stack.to_owned()
            }
        },
        Operations::Block => {codeblock(&mut stack.to_owned(), op, x, y)}
        _ => stack.to_owned()
    }
}


pub(crate) fn find_string(stack: &mut Stack<Type>) -> Stack<Type> {
    // Remove the top element and store it
    let c = stack.pop().unwrap_or_else(|| String_(String::new()));

    // Skips if the stack is empty
    if c.is_empty() {
        Stack { elements: vec![] }
    } else if !is_op(c.type_to_string_trimmed().as_str())
        && (c.is_string() || is_string_number(c.type_to_string_trimmed().as_str()))
    {
        Stack { elements: vec![c] }
    } else {
        find_string(stack)
    }
}
