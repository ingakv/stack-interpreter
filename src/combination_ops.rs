use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedNumberOrBoolean, ExpectedNumberStringOrList};
use crate::list_codeblock_ops::{codeblock, list};
use crate::stack::Operators::{Equal, Length, Not};
use crate::stack::Type::{Block_, Bool_, Float_, Int_, List_, String_};
use crate::stack::{Operators, Stack, Type};
use crate::string_ops::stack_io;
use std::ops::Neg;

pub(crate) fn combination_ops(input: String) -> Option<Operators> {
    let res = match input.as_str() {
        "length" => {Length},
        "==" => {Equal},
        "not" => {Not},
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
    match stack.last() {
        // If it is a list
        Some(List_(elem)) => { list(Length, List_(elem), None) }

        // If it is a code block
        Some(Block_(elem)) => { codeblock(Length, List_(vec![]), Block_(elem)) }

        _ => { stack_io(Length, (stack.last(), None)) }
    }
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

    if let (Some(some_elem1), Some(some_elem2)) = (elem1.to_owned(), elem2.to_owned()) {

        let mut is_equal = Bool_(elem1 == elem2);
        
        if types.number {
            // This ensures that i.e., 10.0 and 10 are considered as equal
            if let (Some(elem1_float), Some(elem2_float)) = 
                (some_elem1.type_to_float(), some_elem2.type_to_float())
            { is_equal = Bool_(elem1_float == elem2_float); }
        }
        (vec![some_elem1, some_elem2], vec![is_equal])

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
                    let new_elem2 = if el { Bool_(false) }
                    else { Bool_(true) };
                    new_elem2
                }
                _ => { Type::default() }
            };
            rem_vec.push(elem);
        }
        else { print_error(ExpectedNumberOrBoolean); }
        return (rem_vec, vec![new_el])
    }
}


