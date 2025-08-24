use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedString, NotEnoughValues, StackEmpty};
use crate::get_line;
use crate::stack::Type::{Int_, List_, String_};
use crate::stack::{string_to_type, Stack, Type};
use crate::string_ops::StringOnlyOps::{Print, StackFloat, StackInt, Words};

pub(crate) const STACK_OPS: [&str; 3] = ["dup", "swap", "pop"];

#[derive(Clone, Copy)]
pub enum StringOnlyOps {
    Print,
    Words,
    StackInt,
    StackFloat,
}
pub(crate) const IO_OPS: [&str; 2] = ["print", "read"];

pub(crate) const STRING_OPS: [&str; 3] = ["parseinteger", "parsefloat", "words"];




// Performs string-only operations
fn parse_string(operator: StringOnlyOps, elem: Type) -> Type {
    
    match operator {
        
        // Prints the top string to standard output and removes it from the stack
        Print => { elem.print(); String_(String::new()) }

        // Divides the string into words and puts them in a list
        Words => {

            let st = elem.type_to_string_trimmed();
            let str_val: Vec<&str> = st.split_whitespace().collect();
            let mut new_li = vec![];

            for i in str_val { new_li.push(string_to_type(i)); }
            List_(new_li)
        
        }
        _ => { elem }
    }
}

// Parses a string from the stack to a specific type
fn parse_string_from_stack(stack: &mut Stack<Type>, parse_type: StringOnlyOps) -> Stack<Type> {
    
    let mut parsed = false;
    
    // Iterate through the stack from the end
    for elem in stack.to_owned().elements.iter().rev() {

        // Skip if the element is not a string
        if let String_(elem) = elem {
            let mut str = string_to_type(elem);

            // Try to parse the string to the given type
            if str.same_type(parse_type) {

                // If the parse type is a string, perform string operations
                if str.is_string() { str = parse_string(parse_type, str); }
                
            stack.replace_last_match(vec![String_(elem.to_string())], str);
                
                parsed = true;
                
                break;
            }

        }
    }

    if !parsed {print_error(ExpectedString) }
    stack.to_owned()
    
}

pub(crate) fn stack_string_io(elem: &str, stack: &mut Stack<Type>) -> Stack<Type> {

    // Error handling on empty stack
    let len = stack.len();
    if len == 0 { print_error(StackEmpty) }
    else {
        
        match elem {

            // Duplicates the top element
            "dup" => {
                stack.push(stack.last().unwrap())
            }

            // Swaps the top two elements
            "swap" => {
                if len > 1 {
                    stack.swap((len - 2).try_into().unwrap(), (len - 1).try_into().unwrap());
                } else {
                    print_error(NotEnoughValues)
                }
            }

            // Removes the top element
            "pop" => { stack.pop(); }
            
            // Converts a string to an integer
            "parseinteger" => { parse_string_from_stack(stack, StackInt); }

            // Converts a string to a float
            "parsefloat" => { parse_string_from_stack(stack, StackFloat); }

            "words" => { parse_string_from_stack(stack, Words); }

            // Returns the length of the string
            "length" => {
                let st = stack.pop().unwrap_or_else(|| String_(String::new()));
                stack.push(Int_(st.type_to_string_trimmed().len() as i128))
            }

            "print" => { parse_string_from_stack(stack, Print); }

            // Reads an input and adds it to the stack as a string
            "read" => {
                let input = get_line();
                stack.push(String_(input));
            }

            _ => {}
        }
    }

    // Return the stack
    stack.to_owned()
}
