
use crate::string_ops::find_string;

pub(crate) const LIST_OPS: [&str; 9] = [
    "head",
    "tail",
    "empty",
    "length",
    "cons",
    "append",
    "each quotation",
    "map quotation",
    "foldl quotation",
];

pub(crate) fn find_list(stack: &mut Vec<String>, og: &mut Vec<String>) -> Vec<String> {

    let c = if stack.is_empty() { "".to_string() }
    else {
        // Remove top element and store it
        stack.pop().unwrap()
    };


    // Skips if the stack is empty
    if c == "".to_string() {
        vec![]
    }

    // Checks if it is a list
    else if LIST_OPS.contains(&&*c) {
        // Loops through and finds the next lists
        let list = find_list(stack, og);
        let list2 = find_list(stack, og);

        // Loops through and finds the next non-list (AKA string)
        let mut new_li = og.clone();
        new_li.pop();
        let str = find_string(&mut new_li);


        // Ensures that both the list and the string is not empty
        if c == "append" {

            if !list.is_empty() && !str.is_empty() {
                list_op(og, &c, list.first().unwrap(), str.first().unwrap())
            }
            else { og.pop(); og.to_vec() }

        }


        // Ensures that both lists are not empty
        else if c == "cons" {
            if !list.is_empty() && !list2.is_empty() {
                list_op(og, &c, list.first().unwrap(), list2.first().unwrap())
            }
            else { og.pop(); og.to_vec() }
        }


        else if !list.is_empty() {
            list_op(og, &c, list.first().unwrap(), &"".to_owned())
        }

        // If there are no lists in the stack, the original stack gets sent back
        else {
            og.pop();
            og.to_vec()
        }
    }

    else if c.contains("[") {
        vec![c]
    }

    else {
        find_list(stack, og)
    }
}

fn list_op(stack: &mut Vec<String>, c: &str, li: &String, el: &String) -> Vec<String> {

    let mut list: Vec<&str> = li
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split_terminator(',')
        .collect();

    let mut new_li = vec![];
    for i in list.iter() {
        new_li.push(i.trim_matches('\"'));
    }
    list = new_li;

    let new = match c {
        // Returns the first item of the list
        "head" => list.first().unwrap().to_string(),

        // Returns the last item of the list
        "tail" => list.last().unwrap().to_string(),

        // Returns whether or not the list is empty
        "empty" => {
            if list.is_empty() {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }

        // Returns the length of the list
        "length" => list.len().to_string(),


        // Inserts the string onto the front of the list
        "append" => {
            new_li = vec![];
            new_li.push(li.split_at(1).0);
            new_li.push(el);
            new_li.push(", ");
            new_li.push(li.split_at(1).1);
            new_li.concat()
        }


        // Combines the two lists
        "cons" => {
            // Ignores the brackets between the lists
            new_li = vec![li.split_at(li.len()-1).0, ", ", el.split_at(1).1];
            new_li.concat()
        }

        _ => panic!("Invalid input!"),
    };

    // Ensures that if there are duplicates of the predicates, the ones removed are the ones in the back
    stack.reverse();

    if c == "append" || c == "cons" {
        if let Some(str_ref) = stack.iter().position(|r| r == li) {
            stack.remove(str_ref);
        }

        if let Some(str_ref) = stack.iter().position(|r| r == el) {
            stack.remove(str_ref);
        }
    }

    // Reverse it back
    stack.reverse();

    // Removes the operator and adds the new variable
    stack.pop();
    stack.push(new);



    // Return the stack
    stack.to_owned()

}
