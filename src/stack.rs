use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedNumber, StackEmpty};
use crate::stack::Type::*;
use crate::string_ops::StringOnlyOps;
use crate::string_ops::StringOnlyOps::{StackFloat, StackInt};
use std::mem::discriminant;
use crate::mylib::is_op;
/////////////////////////////////////////// Type //////////////////////////////////////////////

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Int_(i128),
    Float_(f64),
    Bool_(bool),
    String_(String),
    List_(Vec<Type>),
    Block_(Vec<Type>),
}

// Type functions
impl Type {
    // Returns the variable as a string
    pub fn type_to_string(&self) -> String {
        match self {
            Int_(str) => str.to_string(),
            Float_(str) => {
                if !str.to_string().contains('.') {
                    format!("{}.0", str.to_string())
                } else {
                    str.to_string()
                }
            }
            Bool_(str) => {
                if str.to_string() == "true" {
                    "True".to_string()
                } else {
                    "False".to_string()
                }
            }
            String_(str) => {
                if !is_op(str.as_str()) {
                    ("\"".to_owned() + &str + "\"").to_string()
                } else {
                    str.to_string()
                }
            }
            List_(str) => {
                let mut new_li: Vec<String> = vec!["[".to_string()];

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

            Block_(str) => {
                let mut new_li: Vec<String> = vec![];

                new_li.push("{ ".to_string());

                if !str.is_empty() {
                    for i in str {
                        new_li.push(i.type_to_string());
                        new_li.push(" ".to_string());
                    }
                    new_li.pop();
                }
                new_li.push(" }".to_string());

                new_li.concat()
            }
        }
    }

    // Returns the variable as an int
    pub fn type_to_int(&self) -> Option<i128> {
        match self {
            Int_(val) => Some(*val),
            Float_(val) => Some(*val as i128),
            Bool_(val) => {
                if *val {
                    Some(1i128)
                } else {
                    Some(0i128)
                }
            }
            String_(val) => {
                match string_to_type(val) {
                    Int_(x) => Int_(x).type_to_int(),
                    Float_(x) => Float_(x).type_to_int(),
                    Bool_(x) => Bool_(x).type_to_int(),
                    _ =>  None
                }
            }
            _ => { print_error(ExpectedNumber); Some(0) }
        }
    }

    // Returns the variable as a float
    pub fn type_to_float(&self) -> Option<f64> {
        match self {
            Int_(val) => Some(*val as f64),
            Float_(val) => Some(*val),
            Bool_(val) => {
                if *val {
                    Some(1.0f64)
                } else {
                    Some(0.0f64)
                }
            }
            String_(val) => {
                match string_to_type(val) { 
                    Float_(x) => Float_(x).type_to_float(),
                    Int_(x) => Int_(x).type_to_float(),
                    Bool_(x) => Bool_(x).type_to_float(),
                    _ =>  None
                }
            }
            _ => { print_error(ExpectedNumber); Some(0.0) }
        }
    }
    

    // Returns the variable as a bool
    pub fn is_empty(&self) -> bool {
        match self {
            String_(val) => {
                val.is_empty()
            }
            _ => false,
        }
    }

    // Checks whether the variable is a...
    pub fn is_list(&self) -> bool {
        match self {
            List_(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Bool_(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            String_(_) => true,
            _ => false,
        }
    }

    pub fn is_block(&self) -> bool {
        match self {
            Block_(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Int_(_) => true,
            Float_(_) => true,
            _ => false,
        }
    }

    pub fn same_type(&self, other: StringOnlyOps) -> bool {
        match self {
            Int_(_) => discriminant(&other) == discriminant(&StackInt),
            Float_(_) => discriminant(&other) == discriminant(&StackFloat),
            String_(_) => discriminant(&other) != discriminant(&StackInt) && 
                          discriminant(&other) != discriminant(&StackFloat),
            _ => false,
        }
    }

    // Prints a single variable
    pub fn print(&self) {
        println!("{}", self.type_to_string())
    }
}

/////////////////////////////////////////// Stack //////////////////////////////////////////////

#[derive(PartialEq, Clone)]
pub struct Stack<Type> {
    pub elements: Vec<Type>,
}

// Stack functions
impl Stack<Type> {
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }
    
    // Compares the stack with another and returns true if they are equal
    #[allow(dead_code)]
    pub fn is_equal(&self, old_stack: Stack<Type>) -> bool {

        let mut is_equal = true;
        let mut old = old_stack.clone();
        let mut new = self.clone();

        loop {
            let (Some(old_elem), Some(new_elem)) = (old.pop(), new.pop()) else { break };
            if old_elem != new_elem { is_equal = false; break }
        }
        is_equal
        
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

    pub fn pop_front(&mut self) -> Option<Type> {
        self.reverse();
        let elem = self.elements.pop();
        self.reverse();
        elem
    }

    pub fn pop(&mut self) -> Option<Type> {
        self.elements.pop()
    }

    pub fn swap(&mut self, pos1: usize, pos2: usize) -> Option<Stack<Type>> {
        if self.len() > 1 {
            self.elements.swap(pos1, pos2);
            Some(self.to_owned())
        } else {
            None
        }
    }

    pub fn push(&mut self, i: Type) {
        self.elements.push(i);
    }

    pub fn reverse(&mut self) {
        self.elements.reverse()
    }

    // Removes the last element of the stack that matches the one given
    pub fn replace_last_match(&mut self, mut remove: Vec<Type>, new: Type) -> Self {
        // Only pushes the new value if it isn't empty
        let mut swap = if !new.is_empty() {
            self.reverse();
            self.push(new);
            self.reverse();
            true
        } else {
            false
        };

        while !remove.is_empty() {
            // Ensures that if there are duplicates of the numbers, the ones removed are the ones in the back
            self.reverse();

            if let Some(rem) = remove.pop() {
                if let Some(str_ref) = self.elements.iter().position(|r| r == &rem) {
                    // Swaps the first element in 'remove' with the new element
                    if swap {
                        self.elements.swap_remove(str_ref);
                        swap = false;
                    } else {
                        self.elements.remove(str_ref);
                    }
                }
            };

            // Reverse it back
            self.reverse();
        }

        self.to_owned()
    }

    pub fn print_stack(&self) {
        if !self.is_empty() {
            // Prints the stack
            println!("\nStack: ");
            for i in self.elements.iter().rev() {
                i.print();
            }
        }
    }

    pub fn has_code(&self) -> bool {
        let mut ans = false;

        for i in &self.elements {
            if i.is_block() {
                ans = true
            }
        }
        ans
    }

    #[allow(dead_code)]
    pub fn stack_to_string(&self) -> String {
        if !self.is_empty() {
            let mut buf = vec![];

            for i in self.elements.iter() {
                match i {
                    List_(_) | String_(_) | Block_(_) => {
                        let mut new_li = i.type_to_string();
                        new_li = new_li.replace("[", " [ ");
                        new_li = new_li.replace("{", " { ");
                        new_li = new_li.replace("}", " } ");
                        new_li = new_li.replace("]", " ] ");
                        new_li = new_li.replace(",", " ");
                        new_li = new_li.replace("\"", " \" ");

                        buf.push(new_li)
                    }
                    _ => {
                        buf.push(i.type_to_string());
                        buf.push(" ".to_string());
                    }
                }
            }
            buf.concat()
        } else {
            print_error(StackEmpty);
            String::new()
        }
    }
}



//////////////////////////////////// Additional functions //////////////////////////////////////

// Chooses which type to put the variable in
pub fn string_to_type(var: &str) -> Type {

    // Checks whether the variable is a float
    if is_string_number(var) {
        if var.contains('.') { Float_(var.parse::<f64>().unwrap()) }
        else {Int_(var.parse::<i128>().unwrap())}
    }

    else if var == "True" {Bool_(true)}
    else if var == "False" {Bool_(false)}

    else {String_(var.to_owned())}
}

// Checks whether the variable is a valid number
// Returns true for both ints and floats
pub(crate) fn is_string_number(el: &str) -> bool {

    // Prevents error when checking empty strings
    let mut is_num = !el.is_empty();

    let st: String =  el.split_terminator('.').collect();

    // Loops until a non-digit is found
    for i in st.trim_start_matches('-').as_bytes() {
        if !i.is_ascii_digit() {is_num = false}
    }

    // A minus sign is not a number
    if el.trim().as_bytes() == b"-" { is_num = false; }

    is_num
}



// Checks whether the variable is a quotation
pub(crate) fn is_block(el: Vec<Type>) -> bool {
    for i in el { if !i.is_block() {return false} }
    true
}

// Checks whether the variable is a list
#[allow(dead_code)]
pub(crate) fn is_list(el: Vec<Type>) -> bool {
    for i in el { if !i.is_list() {return false} }
    true
}
