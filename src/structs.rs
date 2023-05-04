
use crate::error_handling::Error::{ExpectedNumber, ProgramFinishedWithMultipleValues, StackEmpty};
use crate::error_handling::print_error;
use crate::mylib::is_string;
use crate::structs::Type::*;



/////////////////////////////////////////// Type //////////////////////////////////////////////


#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Int_(i128),
    Float_(f64),
    Bool_(bool),
    String_(String),
    List_(Vec<Type>),
}

// Type functions
impl Type{

    // Returns the variable as a string
    pub fn type_to_string(&self) -> String {
        match self {
            Int_(str) => {str.to_string()}
            Float_(str) => {
                if !str.to_string().contains('.') { format!("{}.0", str.to_string())}
                else { str.to_string() }
            }
            Bool_(str) => {
                if str.to_string() == "true" { "True".to_string()}
                else { "False".to_string() }
            }
            String_(str) => {"\"".to_string() + str + "\""}
            List_(str) => {

                let mut new_li: Vec<String> = vec![];

                new_li.push("[".to_string());

                if !str.is_empty() {
                    for i in str {
                        new_li.push(i.type_to_string());
                        new_li.push(",".to_string());
                    }
                    new_li.pop();
                }
                new_li.push("]".to_string());

                new_li.concat()

            }
        }
    }

    // Returns the variable as an int
    pub fn type_to_int(&self) -> i128 {
        match self {
            Int_(val) => {*val as i128}
            Float_(val) => {*val as i128}
            _ => {print_error(ExpectedNumber); 0}
        }
    }

    // Returns the variable as a float
    pub fn type_to_float(&self) -> f64 {
        match self {
            Int_(val) => {*val as f64}
            Float_(val) => {*val as f64}
            _ => {print_error(ExpectedNumber); 0.0}
        }
    }

    // Returns the variable as a bool
    pub fn type_to_bool(&self) -> bool {
        match self {
            Bool_(val) => {*val}
            _ => {print_error(ExpectedNumber); panic!()}
        }
    }


    // Prints a single variable
    pub fn print(&self) {
        if is_string(self.to_owned()) { println!("\"{}\"", self.type_to_string()) }
        println!("{}", self.type_to_string())
    }

}


/////////////////////////////////////////// Stack //////////////////////////////////////////////


#[derive(PartialEq, Clone)]
pub struct Stack<Type>{
    pub elements: Vec<Type>
}

// Stack functions
impl Stack<Type>{

    pub fn new() -> Self {
        Stack { elements: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn first(&self) -> Option<Type> {
        self.elements.first().cloned()
    }

    pub fn last(&self) -> Option<Type> {
        self.elements.last().cloned()
    }

    pub fn pop(&mut self) -> Option<Type> {
        self.elements.pop()
    }

    pub fn swap(&mut self, pos1: usize, pos2: usize) -> Option<Stack<Type>> {

        if self.len() > 1 {
            self.elements.swap(pos1, pos2);
            Some(self.to_owned())
        }
        else { None }
    }


    pub fn pop_front(&mut self) -> Option<Type> {

        if !self.is_empty() {
            self.reverse();
            let top = self.pop().unwrap();
            self.reverse();
            Some(top)
        }
        else { None }
    }


    pub fn push(&mut self, i: Type) { self.elements.push(i); }

//    pub fn clone(&self) { self.elements.clone() }

    pub fn reverse(&mut self) { self.elements.reverse() }


    // Removes the last element of the stack that matches the one given
    pub fn remove_last_match(&mut self, el: Type) -> Self {

        // Ensures that if there are duplicates of the numbers, the ones removed are the ones in the back

        self.reverse();

        if let Some(str_ref) = self.elements.iter().position(|r| r == &el) {
            self.elements.remove(str_ref);
        }

        // Reverse it back
        self.reverse();

        self.clone()

    }



    pub fn print_stack(&self) {

        if !self.is_empty() {
            // Prints the stack
            println!("Stack: ");
            for i in self.elements.iter().rev() {
                i.print();
            }
        }
        else { print_error(StackEmpty); }

    }




    pub fn stack_to_string(&self) -> String {

        if !self.is_empty() {
            // Prints the stack
            print_error(ProgramFinishedWithMultipleValues);

            let mut stack = vec![];

            for i in self.elements.iter().rev() {
                stack.push(i.type_to_string());
                stack.push(" ".to_string());
            }
            stack.pop();
            stack.concat()
        }
        else { print_error(StackEmpty); "".to_string() }

    }







}

