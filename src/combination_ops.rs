use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedNumberStringOrList, StackEmpty};
use crate::list_ops::list_op;
use crate::quotation_ops::quotation;
use crate::stack::Type::{Bool_, Float_, Int_, List_, String_};
use crate::stack::{is_block, Stack, Type};
use crate::string_stack_io_ops::parse_string;
use std::ops::Neg;


// By making these separate functions, several datatypes can be compared

// Returns the length of the list or string
pub(crate) fn length(stack: &mut Stack<Type>) -> Stack<Type> {

    if stack.is_empty() { print_error(StackEmpty); return stack.to_owned(); }

    let mut og = stack.clone();

    let elem = stack.pop().unwrap_or_else(|| String_(String::new()));


    // If it is a list
    if let List_(x) = elem.to_owned() {
        list_op(&mut og.to_owned(), "length", x, String_(String::new()))
    }

    // If it is a code block
    else if is_block(vec![elem.to_owned()]) {
        og.replace_last_match(vec![elem.to_owned()], String_(String::new()));
        quotation(&mut og.to_owned(), "length", elem, List_(vec![]))
    }

    else { parse_string("length", &mut og) }

}


pub(crate) fn compare(stack: &mut Stack<Type>) -> Stack<Type> {

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
                    if let Some(Int_(_) | Float_(_)) = elem1.to_owned() { elem2 = Some(elem); break }
                    else if !is_string && !is_list { elem1 = Some(elem); is_number = true; }
                }
                String_(_) => {
                    if let Some(String_(_)) = elem1.to_owned() { elem2 = Some(elem); break }
                    else if !is_number && !is_list { elem1 = Some(elem); is_string = true; }
                }
                List_(_) => {
                    if let Some(List_(_)) = elem1.to_owned() { elem2 = Some(elem); break }
                    else if !is_number && !is_string { elem1 = Some(elem); is_list = true; }
                }
                _ => {}
            }
        }

        else { break }
    }

    if elem1.is_some() && elem2.is_some() {
        
        if is_number {
            // This ensures that i.e., 10.0 and 10 are considered as equal
            elem1 = Some(Float_(elem1.unwrap().type_to_float()));
            elem2 = Some(Float_(elem2.unwrap().type_to_float()));
        }
        stack.push(Bool_(elem1 == elem2));
    }
    else { print_error(ExpectedNumberStringOrList); };

    stack.to_owned()

}

pub(crate) fn invert(stack: &mut Stack<Type>) -> Stack<Type> {

    let mut old_stack = stack.clone();
    loop {

        if let Some(elem) = old_stack.pop() {
            match elem {

                // Turns a negative number positive, or the opposite
                Int_(el) => {
                    stack.replace_last_match(vec![elem.to_owned()], Int_(el.neg())); break; }

                Float_(el) => {
                    stack.replace_last_match(vec![elem.to_owned()], Float_(el.neg())); break; }

                // Inverts the predicate
                Bool_(el) => {
                    let new_elem = if el { Some(Bool_(false)) }
                    else { Some(Bool_(true)) };
                    stack.replace_last_match(vec![elem.to_owned()], new_elem.unwrap()); break;
                }
                _ => {}
            }
        }
        else { break }
    }

    stack.to_owned()
}


