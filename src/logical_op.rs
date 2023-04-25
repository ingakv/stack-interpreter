use crate::mylib::{is_literal};

pub(crate) fn find_logical(stack: &mut Vec<String>, og: &mut Vec<String>) -> Vec<String> {


    let c = if stack.is_empty() {
        "".to_string()
    }
    else {
        // Remove top element and store it
        stack.pop().unwrap()
    };

    let ops = ["&&", "||", "not"];

    // Skips if the stack is empty
    if c == "".to_string() {
        vec![]
    }

    // Checks if it is an operator
    else if ops.contains(&&*c) {
        // Loops through and finds the next two literals
        let num2 = find_logical(stack, og);
        let num1 = find_logical(stack, og);

        if !num1.is_empty() && !num2.is_empty() {
            logical_op(og, &c, num2.first().unwrap(), num1.first().unwrap())
        }

        // If there is only 1 variable, it gets pushed back on, and the stack returns, unless "not" is used
        else if c == "not" {
            logical_op(og, &c, num2.first().unwrap(), num2.first().unwrap())
        }

        // If there are less than two valid numbers in the stack, the original stack gets sent back
        // (without the operator)
        else {
            og.pop();
            og.to_vec()
        }

    }

    else if is_literal(c.clone()) {
        vec![c]
    }

    else {
        find_logical(stack, og)
    }
}




fn logical_op(stack: &mut Vec<String>, c:&str, a: &String, b: &String) -> Vec<String> {


    let x: String = a.to_string();
    let y: String = b.to_string();


    let new = match c {

        // Checks whether both predicates are True or not
        "&&" => {
            if x == "True" && y == "True" { "True".to_string() } else { "False".to_string() }
        },

        // Checks whether at least one of the predicates are True or not
        "||" => {
            if x == "True" || y == "True" { "True".to_string() } else { "False".to_string() }
        },


        // Inverts the predicate
        "not" => {
            if x == "True" { "False".to_string() } else { "True".to_string() }
        }

        _ => panic!("Invalid input!")
    };



    // Ensures that if there are duplicates of the predicates, the ones removed are the ones in the back
    stack.reverse();

    if let Some(str_ref) = stack.iter().position(|r| r == &x) {
        stack.remove(str_ref);
    }
    if c != "not" {
        if let Some(str_ref) = stack.iter().position(|r| r == &y) {
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

