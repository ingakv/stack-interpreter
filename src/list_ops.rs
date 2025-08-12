use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedList, ExpectedListOrString};
use crate::stack::Type::{Bool_, Int_, List_, String_};
use crate::stack::{Stack, Type};
use crate::string_stack_io_ops::find_string;

pub(crate) const LIST_OPS: [&str; 5] = ["head", "tail", "empty", "cons", "append"];

pub(crate) fn find_list(stack: &mut Stack<Type>, og: &mut Stack<Type>, skip: bool) -> Stack<Type> {
    let c = stack
        .elements
        .pop()
        .unwrap_or_else(|| String_(String::new()));

    let st = c.type_to_string();
    let op = st.trim_start_matches("\"").trim_end_matches("\"");

    // Skips if the stack is empty
    if c.is_empty() {
        Stack::new()
    }
    // Checks if it is a list
    else if LIST_OPS.contains(&op) && !skip {
        // Loops through and finds the next lists
        let list = find_list(stack, og, true);
        let list2 = find_list(stack, og, true);

        // Loops through and finds the next non-list (AKA string)
        let mut new_li = og.clone();
        new_li.elements.pop();
        let str = find_string(&mut new_li);

        if let Some(List_(x)) = list.first() {
            // Functions with two lists
            if let Some(y) = list2.first() {
                match op {
                    "append" | "cons" => list_op(og, &op, x, y),
                    _ => list_op(og, &op, x, String_(String::new())),
                }
            }
            // Functions with a list and a string
            else if let Some(y) = str.first() {
                match op {
                    "append" => list_op(og, &op, x, y),
                    _ => list_op(og, &op, x, y),
                }
            }
            // Functions that require only one list
            else {
                match op {
                    // This is to return the value to quotation_ops
                    "each" => Stack {
                        elements: x.to_owned(),
                    },
                    _ => list_op(og, &op, x.to_owned(), String_(String::new())),
                }
            }
        }
        // Ensures that both the list and the string / list2 is not empty
        else if op == "append" {
            print_error(ExpectedListOrString);
            og.pop();
            og.to_owned()
        }
        // Ensures that both lists are not empty
        else if op == "cons" {
            print_error(ExpectedList);
            og.pop();
            og.to_owned()
        }
        // If there are no lists in the stack, the original stack gets sent back
        else {
            print_error(ExpectedList);
            og.pop();
            og.to_owned()
        }
    } else if c.is_list() {
        Stack { elements: vec![c] }
    } else {
        find_list(stack, og, true)
    }
}

pub(crate) fn list_op(stack: &mut Stack<Type>, c: &str, li: Vec<Type>, el: Type) -> Stack<Type> {
    // Removes the operator
    stack.pop();

    let head = if !li.is_empty() {
        li.first().unwrap().to_owned()
    } else {
        String_(String::new())
    };

    let new_el = match c {
        // Returns the first item of the list
        "head" => head,

        // Returns the last item of the list
        "tail" => {
            let mut new_li = vec![];
            for i in li.to_owned() {
                if i != head {
                    new_li.push(i)
                }
            }
            List_(new_li)
        }

        // Returns whether the list is empty
        "empty" => Bool_(li.is_empty()),

        // Returns the length of the list
        "length" => Int_(li.len() as i128),

        // Inserts the string onto the front of the list
        "append" => {
            let mut list = vec![el.to_owned()];

            for i in li.to_owned() {
                list.push(i);
            }

            List_(list)
        }

        // Combines the two lists
        "cons" => {
            // Return the other list if one of them is empty
            if li.is_empty() {
                match el.to_owned() {
                    List_(i) => List_(i),
                    _ => List_(vec![el.to_owned()]),
                }
            } else {
                let mut list = vec![];

                match el.to_owned() {
                    List_(i) => list = i,
                    _ => print_error(ExpectedList),
                }

                for i in li.to_owned() {
                    if !list.contains(&i) {
                        list.push(i);
                    }
                }

                List_(list)
            }
        }

        _ => panic!("An error occurred in list_ops!"),
    };

    let mut rem = li.clone();
    rem.push(el.to_owned());

    // Remove the original numbers or replaces them with the new element
    stack.replace_last_match(vec![List_(li), el], new_el);

    // Return the stack
    stack.to_owned()
}
