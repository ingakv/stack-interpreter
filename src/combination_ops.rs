use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedNumberOrBoolean, ExpectedNumberStringOrList};
use crate::list_codeblock_ops::{codeblock, list};
use crate::stack::Operators::{Equal, Length, Not};
use crate::stack::Type::{Bool_, Float_, Int_, List_, String_};
use crate::stack::{Operators, Stack, Type};
use crate::string_ops::stack_io;
use std::ops::Neg;

pub(crate) fn combination_ops(input: String) -> Option<Operators> {
    let res = match input.as_str() {
        "length" => {Length},
        "==" => {Equal},
        "not" => { Not },
        _ => {return None;}
    };
    Some(res)
}


// By making these separate functions, several datatypes can be compared
pub(crate) fn combination(stack: &mut Stack<Type>, op: Operators) -> Stack<Type> {
    
    let (rem, new) = match op {
        Length => { length(stack) }
        Equal => { compare(stack) }
        Not => { invert(stack) }
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
        list(Length, List_(x), None)
    }

    // If it is a code block
    else if elem.is_block() {
        codeblock(Length, List_(vec![]), elem)
    }

    else { stack_io(Length, (stack.last(), None)) };
    
    (remove_vec, new_el)
}


pub struct  CompareTypes {
    number: bool,
    string: bool,
    list: bool,
    bool: bool,
}
impl CompareTypes {
    fn not_claimed(&self) -> bool { 
        match self { 
            CompareTypes { number: false, string: false, list: false, bool: false } => true,
            _ => false,
        }
    }
}

fn compare(stack: &mut Stack<Type>) -> (Vec<Type>, Vec<Type>) {

    let mut elem1 = None;
    let mut elem2 = None;
    let mut types =  CompareTypes {
        number: false,
        string: false,
        list: false,
        bool: false,
    };

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
                    else if types.not_claimed() { elem1 = Some(elem); types.number = true; }
                }
                String_(_) => {
                    if let Some(String_(_)) = elem1 { elem2 = Some(elem); break }
                    else if types.not_claimed() { elem1 = Some(elem); types.string = true; }
                }
                Bool_(_) => {
                    if let Some(Bool_(_)) = elem1 { elem2 = Some(elem); break }
                    else if types.not_claimed() { elem1 = Some(elem); types.bool = true; }
                }
                List_(_) => {
                    if let Some(List_(_)) = elem1 { elem2 = Some(elem); break }
                    else if types.not_claimed() { elem1 = Some(elem); types.list = true; }
                }
                _ => {}
            }
        }

        else { break }
    }

    if elem1.is_some() && elem2.is_some() {

        let mut is_equal = Bool_(elem1 == elem2);
        
        if types.number {
            // This ensures that i.e., 10.0 and 10 are considered as equal
            let elem1_float = Some(Float_(elem1.to_owned().unwrap().type_to_float().unwrap()));
            let elem2_float = Some(Float_(elem2.to_owned().unwrap().type_to_float().unwrap()));
            is_equal = Bool_(elem1_float == elem2_float);
        }
        (vec![elem1.unwrap_or_default(), elem2.unwrap_or_default()], vec![is_equal])

    }
    else { print_error(ExpectedNumberStringOrList); (vec![], vec![]) }

}

fn invert(stack: &mut Stack<Type>) -> (Vec<Type>, Vec<Type>) {

    let mut rem_vec = vec![];
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


