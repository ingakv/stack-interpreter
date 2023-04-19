use std::io;
use std::io::Write;
use crate::mylib::{check_operator, get_line, push_to_vec};

pub mod mylib;


//mod test;



fn main() {

    let mut stack: Vec<String> = Vec::new();

    loop {
        print!("\nbprog> ");
        io::stdout().flush().unwrap();

        // Reads user input
        let input = get_line();


        let old_stack = stack.clone();

        // Splits up the different input variables
        let new_el: Vec<&str> = { input.split_whitespace().collect() };


        // Variables to help join the elements together
        let mut str_buf: Vec<&str> = vec![];
        let mut is_str: bool = false;


        let mut li_buf: Vec<&str> = vec![];
        let mut is_list: bool = false;

        for i in new_el {

            // If it is the start or the end of a list
            if i.contains('[') {
                is_list = true;

                // Add opening bracket
                li_buf.push("[");
            }

//////////////// List /////////////////

            // If it is the end of the list
            else if i.contains(']') {

                // Remove the last comma
                li_buf.pop();

                // Add closing bracket
                li_buf.push("]");

                // If the list is not a sublist, set is_list to false
                if li_buf.iter().filter(|&n| *n == "[").count() == li_buf.iter().filter(|&n| *n == "]").count() {

                    // Join the vector together to form a list, and send it to the stack
                    check_operator(li_buf.concat().as_str(), &mut stack);

                    is_list = false;

                    // Reset the buffer so that a potential new list can be read
                    li_buf.clear();
                }

                // If the list is a sublist, continue reading it
                else { li_buf.push(","); }

            }


//////////////// String /////////////////

            // If it is the start or the end of a string
            else if i.contains('"') {

                // If it is the end of the string
                if is_str {

                    // Remove the last whitespace
                    str_buf.pop();


                    // Copy the elements into a combined list
                    // If there is no list, there are no extra elements added, so str_buf can get set to the new list

                    str_buf = push_to_vec(li_buf.clone(), str_buf.clone());

                    // If we are in a list, copy the new list over
                    if is_list{

                        li_buf = str_buf.clone();
                        li_buf.pop();
                        li_buf.push(", ")
                    }


                    // Join the vector together to form a sentence / string, and send it to the stack
                    else { check_operator(str_buf.concat().as_str(), &mut stack); }


                    // Reset the buffer so that a potential new string can be read
                    str_buf.clear();

                }

                // Flip the boolean
                is_str = !is_str;

            }

            // If a string is currently being read, push it to the buffer, with a whitespace after
            else if is_str {
                str_buf.push(i);
                str_buf.push(" ");
            }


            // If a list is currently being read, push it to the buffer, with a comma after
            else if is_list {
                li_buf.push(i);
                li_buf.push(",");
            }

            else { stack = check_operator(i, &mut stack); }

        }

        // If nothing changed, display this message
        if stack == old_stack {
            println!("\nSyntax error, try again!\n");
        }

        // Prints the stack
        println!("Stack: ");
        for i in stack.iter().rev() {
            println!("{}",i);
        }

    }

}


