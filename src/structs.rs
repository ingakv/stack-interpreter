use std::any::Any;
use crate::error_handling::Error::{ExpectedNumber, ProgramFinishedWithMultipleValues, StackEmpty};
use crate::error_handling::print_error;
use crate::structs::Type::*;



/////////////////////////////////////////// Type //////////////////////////////////////////////


#[derive(PartialEq, Clone)]
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
            Float_(str) => {str.to_string()}
            Bool_(str) => {str.to_string()}
            String_(str) => {str.to_string()}
            List_(str) => {

                let mut new_li: Vec<String> = vec![];
                for i in str {
                    new_li.push(i.type_to_string());
                    new_li.push(", ".to_string());
                }
                new_li.pop();
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


    pub fn clone(&self) -> Self {
        self.clone()
    }


    // Prints a single variable
    pub fn print(&self) {
        println!("\n{}", self.type_to_string())
    }

    pub fn len(&self) -> usize {
        self.len()
    }


    pub fn is_number(&self) -> bool {
        (self.type_id() == Int_.type_id()) || (self.type_id() == Float_.type_id())
    }

    pub fn is_bool(&self) -> bool {
        self.type_id() == Bool_.type_id()
    }

    pub fn is_string(&self) -> bool {
        self.type_id() == String_.type_id()
    }


    pub fn is_list(&self) -> bool {
        self.type_id() == List_.type_id()
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

    pub fn len(&self) -> i128 {
        self.len()
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty()
    }

    pub fn first(&self) -> Option<Type> {
        self.first()
    }

    pub fn last(&self) -> Option<Type> {
        self.last()
    }

    pub fn pop(&self) -> Option<Type> {
        self.pop()
    }

    pub fn swap(&self, pos1: i128, pos2: i128) -> Option<Type> {
        self.swap(pos1, pos2)
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


    pub fn push(&mut self, i: Type) {
        self.push(i);
    }

    pub fn clone(&self) -> Self {
        self.clone()
    }

    pub fn reverse(&self) -> Self {
        self.reverse()
    }


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
            print_error(ProgramFinishedWithMultipleValues);
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

