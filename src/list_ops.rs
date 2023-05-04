use crate::error_handling::Error::{ExpectedList, ExpectedListOrString};
use crate::error_handling::{print_error};
use crate::string_ops::{find_string};
use crate::structs::{Stack, Type};
use crate::structs::Type::{Bool_, Int_, List_, String_};

pub(crate) const LIST_OPS: [&str; 5] = [
    "head",
    "tail",
    "empty",
    "cons",
    "append",
];




pub(crate) fn find_list(stack: &mut Stack<Type>, og: &mut Stack<Type>) -> Stack<Type> {

    let c = if stack.is_empty() { String_("".to_string()) }

    else {
        // Remove top element and store it
        stack.elements.pop().unwrap_or_else(|| String_("".to_string()))
    };

    let st = c.type_to_string();
    let op = st.trim_start_matches("\"").trim_end_matches("\"");

    // Skips if the stack is empty
    if c == String_("".to_string()) {
        Stack::new()
    }

    // Checks if it is a list
    else if LIST_OPS.contains(&op) {
        // Loops through and finds the next lists
        let list = find_list(stack, og);
        let list2 = find_list(stack, og);

        // Loops through and finds the next non-list (AKA string)
        let mut new_li = og.clone();
        new_li.elements.pop();
        let str = find_string(&mut new_li);

//        if str.is_empty() && list2.is_empty() && !stack.is_empty() { str = Stack { vec![stack.elements.pop().unwrap_or_else(|| String_("".to_string()));]} }



        // Ensures that both the list and the string / list2 is not empty
        if op == "append" {


            if let (Some(List_(x)), Some(y)) = (list.first(), str.first()) {

                do_list_op(og, &op, list, str, x, y)
            }

            else if let (Some(List_(x)), Some(y)) = (list.first(), list2.first()) {

                do_list_op(og, &op, list, list2, x, y)
            }

            else { print_error(ExpectedListOrString); og.pop(); og.clone() }

        }

        // Ensures that both lists are not empty
        else if op == "cons" {
            if let (Some(List_(x)), Some(y)) = (list.first(), list2.first()) {

                do_list_op(og, &op, list, list2, x, y)
            }
            else { print_error(ExpectedList); og.pop(); og.clone() }
        }


        else if !list.is_empty() {

            if let Some(List_(x)) = list.first() {

                do_list_op(og, &op, list.clone(), list.clone(), x, String_("".to_owned()))
            }
            else { print_error(ExpectedList); og.pop(); og.clone() }

        }

        // If there are no lists in the stack, the original stack gets sent back
        else {
            print_error(ExpectedList);
            og.pop();
            og.clone()
        }
    }

    else if c. is_list(){
        Stack{ elements: vec![c] }
    }

    else {
        find_list(stack, og)
    }
}

fn do_list_op(stack: &mut Stack<Type>, c: &str, list: Stack<Type>, list2: Stack<Type>, li: Vec<Type>, el: Type) -> Stack<Type> {

    let len = list_op(stack, c, li, el);

    stack.remove_last_match(list.first().unwrap().to_owned());
    stack.remove_last_match(list2.first().unwrap().to_owned());

    len
}


pub(crate) fn list_op(stack: &mut Stack<Type>, c: &str, li: Vec<Type>, el: Type) -> Stack<Type> {

    let head =
        if !li.is_empty() { li.first().unwrap().clone() }
        else { String_("".to_owned()) };


    // Removes the operator and adds the new variable
    stack.pop();

    match c {
        // Returns the first item of the list
        "head" => { stack.push(head) },

        // Returns the last item of the list
        "tail" => {
            let mut new_li = vec![];
            for i in li {
                if i != head {new_li.push(i)}
            }
            stack.push(List_(new_li));
        },

        // Returns whether or not the list is empty
        "empty" => stack.push(Bool_(li.is_empty())),

        // Returns the length of the list
        "length" => stack.push(Int_(li.len() as i128)),


        // Inserts the string onto the front of the list
        "append" => {

            let mut list: Vec<Type> = vec![];

            list.push(el);

            for i in li {
                list.push(i);
            }

            stack.push(List_(list));

        }


        // Combines the two lists
        "cons" => {

            // Return the other list if one of them is empty
            if li.is_empty() { stack.push(el); }

            else {

                let mut list = vec![];

                match el {
                    List_(i) => { list = i }
                    _ => { print_error(ExpectedList) }
                }

                for i in li {
                    if !list.contains(&i) { list.push(i); }
                }

                stack.push(List_(list));

            }
        }

        _ => panic!("An error occurred in list_ops!"),
    };




    // Return the stack
    stack.to_owned()

}
