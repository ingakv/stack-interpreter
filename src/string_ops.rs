use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedString, NotEnoughValues, StackEmpty};
use crate::get_line;
use crate::stack::Type::{Int_, List_, String_};
use crate::stack::{string_to_type, Operators, Stack, Type};
use crate::stack::Operators::{Dup, ParseFloat, ParseInteger, Pop, Print, Read, Swap, Words, Length};

pub(crate) fn stack_io_ops(input: String) -> Option<Operators> {
    let res = match input.as_str() {
        "dup" => {Dup},
        "swap" => {Swap},
        "pop" => {Pop},
        
        "print" => {Print},
        "read" => {Read},
        _ => {return None;}
    };
    Some(res)
}

pub(crate) fn strings_ops(input: String) -> Option<Operators> {
    let res = match input.as_str() {
        "parseinteger" => {ParseInteger},
        "parsefloat" => {ParseFloat},
        "words" => { Words },
        _ => {return None;}
    };
    Some(res)
}

// Performs string-only operations
pub(crate) fn string_ops(op: Operators, stack: &mut Stack<Type>) -> (Vec<Type>, Vec<Type>) {

    // Error handling on empty stack
    if stack.is_empty() { print_error(StackEmpty) }
    else if ![ParseFloat, ParseInteger, Words].contains(&op) { print_error(ExpectedString); }
    else {
        // Iterate through the stack from the end
        for elem in stack.to_owned().elements.iter().rev() {

            // Skip if the element is not a string
            if let String_(elem) = elem {
                let mut str = string_to_type(elem);

                // Parses a string from the stack to a specific type
                if str.same_type(op) {

                    // Divides the string into words and puts them in a list
                    if op == Words {
                        let st = str.type_to_string_trimmed();
                        let str_val: Vec<&str> = st.split_whitespace().collect();
                        let mut new_li = vec![];

                        for i in str_val { new_li.push(string_to_type(i)); }
                        str = List_(new_li)
                    }

                    return (vec![String_(elem.to_string())], vec![str])
                }
            }
        }
    }
    (vec![], vec![])
}

pub(crate) fn stack_io(op: Operators, elems: (Option<Type>, Option<Type>)) -> (Vec<Type>, Vec<Type>) {

    let mut new_el = vec![];
    let mut remove_el = vec![];
    
    let elem = elems.to_owned().0;

    // Error handling on empty stack
    if elem.is_none() { print_error(StackEmpty) }
    else {
        
        match op {

            // Duplicates the top element
            Dup => { new_el.push(elem.unwrap_or_default()) }

            // Swaps the top two elements
            Swap => {
                if elems.1.is_none() { print_error(NotEnoughValues) }
                else {
                    // Gets added back in the opposite order
                    new_el.push(elems.to_owned().0.unwrap());
                    new_el.push(elems.to_owned().1.unwrap());
                    remove_el.push(elems.1.unwrap());
                    remove_el.push(elems.0.unwrap());
                }
            }

            // Removes the top element
            Pop => { remove_el.push(elem.unwrap_or_default()) }

            // Returns the length of the string
            Length => {
                let st = elem.unwrap_or_default();
                remove_el.push(st.to_owned());
                new_el.push(Int_(st.type_to_string_trimmed().len() as i128))
            }

            // Prints the top string to standard output and removes it from the stack
            Print => { 
                let st = elem.unwrap_or_default();
                st.print();
                remove_el.push(st.to_owned())
            }

            // Reads an input and adds it to the stack as a string
            Read => {
                let input = get_line();
                new_el.push(String_(input))
            }

            _ => { print_error(ExpectedString) }
        }
    }

    (remove_el, new_el)
}
