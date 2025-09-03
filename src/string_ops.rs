use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedString, NotEnoughValues, StackEmpty};
use crate::get_line;
use crate::stack::Type::{Int_, List_, String_};
use crate::stack::{string_to_type, Stack, Type};
use crate::string_ops::StringOnlyOps::{StackFloat, StackInt, Words};

pub(crate) const STACK_OPS: [&str; 3] = ["dup", "swap", "pop"];

pub(crate) const IO_OPS: [&str; 2] = ["print", "read"];

pub(crate) const STRING_OPS: [&str; 3] = ["parseinteger", "parsefloat", "words"];


#[derive(Clone, Copy)]
pub enum StringOnlyOps {
    Words,
    StackInt,
    StackFloat,
}

impl StringOnlyOps {
    pub fn is_words(&self) -> bool {
        match self {
            Words => true,
            _ => false
        }
    }
}


// Performs string-only operations
fn parse_string(stack: &mut Stack<Type>, parse_type: StringOnlyOps) -> (Vec<Type>, Vec<Type>) {
    
    // Iterate through the stack from the end
    for elem in stack.to_owned().elements.iter().rev() {

        // Skip if the element is not a string
        if let String_(elem) = elem {
            let mut str = string_to_type(elem);

            // Parses a string from the stack to a specific type
            if str.same_type(parse_type) {

                // Divides the string into words and puts them in a list
                if parse_type.is_words() {
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

    print_error(ExpectedString);
    (vec![], vec![])
}

pub(crate) fn string_ops(op: &str, stack: &mut Stack<Type>) -> (Vec<Type>, Vec<Type>) {

    // Error handling on empty stack
    if stack.is_empty() { print_error(StackEmpty) }
    else {
        
        match op {
            
            // Converts a string to an integer
            "parseinteger" => { return parse_string(stack, StackInt); }

            // Converts a string to a float
            "parsefloat" => { return parse_string(stack, StackFloat); }

            "words" => { return parse_string(stack, Words); }

            _ => { print_error(ExpectedString); }
        }
    }
    (vec![], vec![])
}

pub(crate) fn stack_io(op: &str, elems: (Option<Type>, Option<Type>)) -> (Vec<Type>, Vec<Type>) {

    let mut new_el = vec![];
    let mut remove_el = vec![];
    
    let elem = elems.to_owned().0;

    // Error handling on empty stack
    if elem.is_none() { print_error(StackEmpty) }
    else {
        
        match op {

            // Duplicates the top element
            "dup" => {
                new_el.push(elem.unwrap_or_default())
            }

            // Swaps the top two elements
            "swap" => {
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
            "pop" => { remove_el.push(elem.unwrap_or_default()) }

            // Returns the length of the string
            "length" => {
                let st = elem.unwrap_or_default();
                remove_el.push(st.to_owned());
                new_el.push(Int_(st.type_to_string_trimmed().len() as i128))
            }

            // Prints the top string to standard output and removes it from the stack
            "print" => { 
                let st = elem.unwrap_or_default();
                st.print();
                remove_el.push(st.to_owned())
            }

            // Reads an input and adds it to the stack as a string
            "read" => {
                let input = get_line();
                new_el.push(String_(input))
            }

            _ => { print_error(ExpectedString) }
        }
    }

    (remove_el, new_el)
}
