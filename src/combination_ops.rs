use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedNumberOrBoolean, ExpectedNumberStringOrList};
use crate::list_codeblock_ops::{codeblock, list_op};
use crate::stack::Type::{Bool_, Float_, Int_, List_, String_, Variable};
use crate::stack::{Stack, Type};
use crate::string_ops::stack_io;
use std::ops::Neg;


pub(crate) const COMBINATION_OPS: [&str; 3] = ["length", "==", "not"];


// By making these separate functions, several datatypes can be compared
pub(crate) fn combination_op(stack: &mut Stack<Type>) -> Stack<Type> {
    // Removes the operator from the stack
    let operator = stack.pop().unwrap();
    let op = operator.type_to_string_trimmed().to_lowercase();
    
    let (rem, new) = match op.as_str() {
        "length" => { length(stack) }
        "==" => { compare(stack) }
        "not" => { invert(stack) }
        _ => { (vec![], vec![]) }
    };
    
    // Removes the operator, the original numbers or replaces them with the new element
    stack.replace_last_match(rem, new);

    // Return the stack
    stack.to_owned()
}



// Returns the length of the list or string
fn length(stack: &mut Stack<Type>) -> (Vec<Type>, Vec<Type>) {

    let elem = stack.last().unwrap_or_default();

    let (remove_vec, new_el);

    // If it is a list
    (remove_vec, new_el) = if let List_(x) = elem {
        list_op("length".to_string(), List_(x), None)
    }

    // If it is a code block
    else if elem.is_block() {
        codeblock("length".to_string(), List_(vec![]), elem)
    }

    else { stack_io("length", (stack.last(), None)) };
    
    (remove_vec, new_el)
}


fn compare(stack: &mut Stack<Type>) -> (Vec<Type>, Vec<Type>) {

    let mut elem1 = None;
    let mut elem2 = None;
    let mut is_number = false;
    let mut is_string = false;
    let mut is_list = false;

    let mut old_stack = stack.clone();

    // Set elem1 and elem2 to be the next 2 numbers or strings in the stack
    loop {

        if let Some(elem) = old_stack.pop() {
            match elem {
                // When the first element that is either a string or a number is found,
                // set elem1 to be the element and set the corresponding boolean to true
                // This ensures that elem1 and elem2 are both either strings or numbers
                Int_(_) | Float_(_) => {
                    if let Some(Int_(_) | Float_(_)) = elem1 { elem2 = Some(elem); break }
                    else if !is_string && !is_list { elem1 = Some(elem); is_number = true; }
                }
                String_(_) => {
                    if let Some(String_(_)) = elem1 { elem2 = Some(elem); break }
                    else if !is_number && !is_list { elem1 = Some(elem); is_string = true; }
                }
                List_(_) => {
                    if let Some(List_(_)) = elem1 { elem2 = Some(elem); break }
                    else if !is_number && !is_string { elem1 = Some(elem); is_list = true; }
                }
                _ => {}
            }
        }

        else { break }
    }

    if elem1.is_some() && elem2.is_some() {

        let mut is_equal = Bool_(elem1 == elem2);
        
        if is_number {
            // This ensures that i.e., 10.0 and 10 are considered as equal
            let elem1_float = Some(Float_(elem1.to_owned().unwrap().type_to_float().unwrap()));
            let elem2_float = Some(Float_(elem2.to_owned().unwrap().type_to_float().unwrap()));
            is_equal = Bool_(elem1_float == elem2_float);
        }
        (vec![Variable("compare".to_string()), elem1.unwrap_or_default(), elem2.unwrap_or_default()], vec![is_equal])

    }
    else { print_error(ExpectedNumberStringOrList); (vec![], vec![]) }

}

fn invert(stack: &mut Stack<Type>) -> (Vec<Type>, Vec<Type>) {

    let mut rem_vec = vec![Variable("not".to_string())];
    let mut new_el = Type::default();
    
    let mut old_stack = stack.clone();
    loop {

        if let Some(elem) = old_stack.pop() {
            new_el = match elem {

                // Turns a negative number positive, or the opposite
                Int_(el) => { Int_(el.neg())}

                Float_(el) => { Float_(el.neg()) }

                // Inverts the predicate
                Bool_(el) => {
                    let new_elem2 = if el { Some(Bool_(false)) }
                    else { Some(Bool_(true)) };
                    new_elem2.unwrap()
                }
                _ => { Type::default() }
            };
            rem_vec.push(elem);
        }
        else { print_error(ExpectedNumberOrBoolean); }
        return (rem_vec, vec![new_el])
    }
}


