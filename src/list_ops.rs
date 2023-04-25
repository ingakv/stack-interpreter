
pub(crate) const LIST_OPS: [&str; 9] = ["head", "tail", "empty", "length", "cons", "append", "each quotation", "map quotation", "foldl quotation"];


pub(crate) fn find_list(stack: &mut Vec<String>, og: &mut Vec<String>) -> Vec<String> {

    let c = if stack.is_empty() { "".to_string() }
    else {
        // Remove top element and store it
        stack.pop().unwrap()
    };

    // Checks if it is a list
    if LIST_OPS.contains(&&*c) {

        // Loops through and finds the next two numbers
        let list = find_list(stack, og);

        if !list.is_empty() {
            list_op(og, &c, list.first().unwrap())
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



fn list_op(stack: &mut Vec<String>, c:&str, x: &String) -> Vec<String> {

    let mut list: Vec<&str> = x.trim_start_matches('[').trim_end_matches(']').split_terminator(',').collect();

    let mut new_li = vec![];
    for i in list.iter() {
        new_li.push(i.trim_matches('\"'));
    }
    list = new_li;


    let new = match c {

        // Returns the first item of the list
        "head" => {
            list.first().unwrap().to_string()
        },

        // Returns the last item of the list
        "tail" => {
            list.last().unwrap().to_string()
        },

        // Returns whether or not the list is empty
        "empty" => {
            if list.is_empty() { "True".to_string() } else { "False".to_string() }
        },

        _ => panic!("Invalid input!")

    };

    // Return the stack
    stack.push(new);

    stack.to_owned()

}


