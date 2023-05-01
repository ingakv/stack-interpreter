
use crate::error_handling::{print_error};
use crate::error_handling::Error::{DivisionByZero, ExpectedNumber};
use crate::mylib::is_number;

pub(crate) const ARITHMETIC_OPS: [&str; 7] = ["+", "-", "*", "/", "div", "<", ">"];



// By making this a separate function, several datatypes can be compared
pub(crate) fn compare(stack: &mut Vec<String>) -> Vec<String> {

    let num1 = stack.pop().unwrap();
    let num2 = stack.pop().unwrap();

    // This ensures that ie 10.0 and 10 is considered as equal
    let ans = if is_number(num1.clone()) && is_number(num2.clone()) {
        let v1: f64 = num1.parse().unwrap();
        let v2: f64 = num2.parse().unwrap();

        (if v1 == v2 { "True" } else { "False" }).to_string()
    }

    else { (if num1 == num2 { "True" } else { "False" }).to_string() };


    stack.push(ans);
    stack.to_owned()

}


pub(crate) fn find_arithmetic(stack: &mut Vec<String>, og: &mut Vec<String>) -> Vec<String> {

    let c = if stack.is_empty() {
        "".to_string()
    }

    else {
        // Remove top element and store it
        stack.pop().unwrap()
    };

    // Skips if the stack is empty
    if c == "".to_string() {
        vec![]
    }

    // Checks if it is an operator
    else if ARITHMETIC_OPS.contains(&&*c) {
        // Loops through and finds the next two numbers
        let num2 = find_arithmetic(stack, og);
        let num1 = find_arithmetic(stack, og);

        if !num1.is_empty() && !num2.is_empty() {
            arithmetic(og, &c, num1.first().unwrap(), num2.first().unwrap())
        }
        // If there are less than two valid numbers in the stack, the original stack gets sent back
        // (without the operator)
        else {
            print_error(ExpectedNumber);
            og.pop();
            og.to_vec()
        }

    }

    else if is_number(c.clone()) {
        vec![c]
    }

    else {
        find_arithmetic(stack, og)
    }
}


fn arithmetic(stack: &mut Vec<String>, c: &str, x: &String, y: &String) -> Vec<String> {

    let v1: f64 = x.parse().unwrap();
    let v2: f64 = y.parse().unwrap();

    // Calculates the answers to the arithmetic operations
    let mut new = match c {
        // Addition
        "+" => (v1 + v2).to_string(),

        // Subtraction
        "-" => (v1 - v2).to_string(),

        // Multiplication
        "*" => (v1 * v2).to_string(),

        // Floating point division
        "/" => {
            if v2 == 0.0 {
                print_error(DivisionByZero);
                stack.push(x.to_string());
                stack.push(y.to_string());
                "".to_string()
            }

            else { (v1 / v2).to_string() }
        },

        // Integer division
        "div" => {
            let a = v1 as i64;
            let b = v2 as i64;

            if b == 0 {
                print_error(DivisionByZero);
                stack.push(x.to_string());
                stack.push(y.to_string());
                "".to_string()
            }
            else { (a / b).to_string() }
        },

        // Smaller than
        "<" => (if v1 < v2 { "True" } else { "False" }).to_string(),

        // Bigger than
        ">" => (if v1 > v2 { "True" } else { "False" }).to_string(),

        _ => panic!("Invalid input!"),
    };

    // Turns the answer into a float if it is an even number and at least one of the variables is a float
    if  c == "/" || ((x.contains(".0") || y.contains(".0")) && (c == "+" || c == "-" || c == "*")) {
        new.push('.');
        new.push('0');
    }

    // Ensures that if there are duplicates of the numbers, the ones removed are the ones in the back
    stack.reverse();

    if let Some(str_ref) = stack.iter().position(|r| r == x) {
        stack.remove(str_ref);
    }
    if let Some(str_ref) = stack.iter().position(|r| r == y) {
        stack.remove(str_ref);
    }

    // Reverse it back
    stack.reverse();

    // Removes the operator and adds the new variable
    stack.pop();
    if new != "" { stack.push(new); }

    stack.to_owned()
}
