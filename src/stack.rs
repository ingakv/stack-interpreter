use crate::error_handling::print_error;
use crate::error_handling::Error::{ExpectedNumber, UnexpectedError};
use crate::find_ops::is_op;
use crate::stack::DataTypes::{BlockType, ListType, StringType};
use crate::stack::Type::{Block_, Bool_, Float_, Int_, List_, String_, Variable};
use crate::string_ops::StringOnlyOps;
use crate::string_ops::StringOnlyOps::{StackFloat, StackInt};
use std::mem::discriminant;
use std::{io, vec};
/////////////////////////////////////////// Type //////////////////////////////////////////////

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Int_(i128),
    Float_(f64),
    Bool_(bool),
    String_(String),
    List_(Vec<Type>),
    Block_(Vec<Type>),
    Variable(String),
}

impl Default for Type {
    fn default() -> Self { String_(String::new()) }
}

// Type functions
impl Type {

    pub fn push(&mut self, elem: Type) {
        match self {
            List_(vec) | Block_(vec) => {
                if let String_(elem_str) = elem.to_owned() {
                    match elem_str.as_str() {
                        "[" => { vec.push(List_(Vec::new())); return; },
                        "{" => { vec.push(Block_(Vec::new())); return; },
                        "]" | "}" => { return; },
                        _ => {vec.push(String_(trim(elem_str))); return; }
                    }
                }
                vec.push(elem);
            }
            String_(str) => {
                // Add a whitespace between elements / words
                let mut vec =
                    if !str.is_empty() { vec![str.to_owned(), " ".to_owned()] }
                    else { vec![] };

                match elem { 
                    String_(el) |
                    Variable(el) => { vec.push(el); }
                    Int_(el) => { vec.push(el.to_string()); }
                    Float_(el) => { vec.push(el.to_string()); }
                    Bool_(el) => { vec.push(el.to_string()); }
                    _ => {}
                }
                *str = vec.concat();
            }
            _ => {}
        }
    }


    // Returns the variable as a string
    pub fn type_to_string(&self) -> String {
        match self {
            Int_(str) => str.to_string(),
            Float_(str) => {
                if !str.to_string().contains('.') {
                    format!("{}.0", str.to_string())
                } else { str.to_string() }
            }
            Bool_(str) => {
                if str.to_string().to_lowercase() == "true" { "True".to_string() }
                else { "False".to_string() }
            }
            String_(str) => { ("\"".to_owned() + &str + "\"").to_string() }
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
                        new_li.push(i.type_to_string_trimmed());
                        new_li.push(" ".to_string());
                    }
                    new_li.pop();
                }
                new_li.push(" }".to_string());

                new_li.concat()
            }
            Variable(str) => {str.to_string()}
        }
    }

    pub fn type_to_string_trimmed(&self) -> String {
        self.type_to_string().trim_matches(|c| c == ' ' || c == '"').to_string()
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

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
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

    pub fn second_to_last(&mut self) -> Option<Type> {
        if self.len() > 1 {
            let mut copy = self.clone();
            copy.pop();
            copy.pop()
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
    pub fn replace_last_match(&mut self, mut remove: Vec<Type>, new: Vec<Type>) -> Self {

        while !remove.is_empty() {
            // Ensures that if there are duplicates of the numbers, the ones removed are the ones in the back
            self.reverse();

            if let Some(rem) = remove.pop() {
                if let Some(str_ref) = self.elements.iter().position(|r| r == &rem) {
                    self.elements.remove(str_ref);
                }
            };

            // Reverse it back
            self.reverse();
        }

        // Only pushes the new value if it isn't empty
        if !new.is_empty() {
            for i in new.iter() { self.push(i.to_owned()); }
        }

        self.to_owned()
    }

    pub fn print_stack(&self) {
        if !self.is_empty() {
            // Prints the stack
            println!("\nStack: ");
            for i in self.elements.iter().rev() { i.print(); }
        }
    }
}

///////////////////////////////////////// Buffers //////////////////////////////////////////////



#[derive(Clone, Copy)]
pub enum DataTypes {
    ListType,
    BlockType,
    StringType,
}


impl DataTypes {

    pub fn is_list_type(&self) -> bool {
        match self {
            ListType => true,
            _ => false,
        }
    }

    pub fn is_block_type(&self) -> bool {
        match self {
            BlockType => true,
            _ => false,
        }
    }

    pub fn is_string_type(&self) -> bool {
        match self {
            StringType => true,
            _ => false,
        }
    }
}


#[derive(Clone)]
pub struct Buffers {
    pub(crate) string: String,
    pub(crate) block: Vec<Type>,
    pub(crate) list: Vec<Type>,
    pub(crate) nested_elements: Vec<DataTypes>,
}

impl Default for Buffers {
    fn default() -> Self {
        Buffers {
            string: String::new(),
            block: Vec::new(),
            list: Vec::new(),
            nested_elements: Vec::new(),
        }
    }
}

// Finds the current target buffer and its type, plus the next outer data type if any
pub(crate) fn find_target_buffer(buffers: &mut Buffers) -> (Option<Type>, Option<DataTypes>) {

    // Check if there is a nested element to determine the current context
    if let Some(elem) = buffers.nested_elements.last() {
        let mut str = buffers.string.clone();

        // Select the correct buffer based on the data type
        let buf =
            if elem.is_block_type() { buffers.block.pop() }
            else if elem.is_list_type() { buffers.list.pop() }
            else if elem.is_string_type() { 
                buffers.string.clear(); 
                if str.ends_with(" ") {str.pop();}
                Some(String_(str)) 
            }
            else {
                // Handle unexpected data type
                print_error(UnexpectedError);
                Some(String_(str))
            }.to_owned();

        // Return the current data type and buffer, and the next outer data type if it exists
        let next = buffers.nested_elements.iter().rev().nth(1);

        (buf, next.cloned())
    } else { (None, None) }
}


// Converts a vector of `Type` elements into the appropriate container type
// and pushes it to the corresponding buffer in `buffers`.
pub(crate) fn datatype_to_type(dt: DataTypes, elem: Type, buffers: &mut Buffers) {
    match dt {
        // If the data type is a list
        ListType => {
            // Try to pop the last list from the buffer
            let new_li: Type =

                if let Some(mut li)  = buffers.list.pop() {
                    // Add the new element to the existing list
                    li.push(elem);
                    li.to_owned()

                // If no existing list, create a new one from elem
                } else { elem };
            buffers.list.push(new_li);
        }
        // If the data type is a block, wrap elem in Block_ and push to buffer
        BlockType => { buffers.block.push(elem) }
        // If the data type is a string, join all elements as a string and push
        StringType => {
            if let String_(str) = elem {
                let vec = vec![buffers.string.to_owned(), " ".to_string(), str];
                String_(trim(vec.concat()));
            }
        }
    }
}


//////////////////////////////////// Additional functions //////////////////////////////////////

// Moves a code block or element from the buffer to the correct place (buffer or stack)
pub(crate) fn push_block_to_buffer(stack: &mut Stack<Type>, buffers: &mut Buffers) {

    // Try to find the current target buffer and its type, plus the next outer data type
    if let (Some(target_buffer), nested) = find_target_buffer(buffers) {

        // Convert the target buffer into the correct type and push to the appropriate buffer
        if let Some(nest_dt) = nested { datatype_to_type(nest_dt, target_buffer, buffers); }
            
        // If there is no nested type, push the last element of the buffer to the stack
        else { stack.elements.push(target_buffer.to_owned()); }

        // Clear the target buffer and remove the last nested element type
        buffers.nested_elements.pop();
    }
}

// Copy the element from the buffer to the correct stack
pub(crate) fn push_to_buffer(stack: &mut Stack<Type>, elem_str: &str, buffers: &mut Buffers) {
    
    if let (Some(mut elem), _) = find_target_buffer(buffers) {
        elem.push(string_to_type(elem_str));
        
        // Returns the correct buffer depending on the data type
        match elem {
            List_(_) => { buffers.list.push(elem) }
            Block_(_) => { buffers.block.push(elem) }
            String_(el) => { buffers.string = el }
            _ => {}
        }
    } else { push_str_to_vec(elem_str, &mut stack.elements); };
}


pub fn string_to_type(var: &str) -> Type {

    // Checks whether the variable is a float
    if is_string_number(var) {
        if var.contains('.') { Float_(var.parse::<f64>().unwrap()) }
        else {Int_(var.parse::<i128>().unwrap())}
    }

    else if var == "True" {Bool_(true)}
    else if var == "False" {Bool_(false)}
    else if is_op(var) {Variable(var.to_owned())}

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

pub(crate) fn get_line() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim_end().to_string()
}

// Pattern matches the types to push to vector
pub(crate) fn push_str_to_vec(i: &str, vec: &mut Vec<Type>) {
    let elem =  trim(i.to_string());
    let new_elem;
    if !elem.is_empty() {
        new_elem =
            match string_to_type(elem.as_str()) {
                Int_(elem) => Int_(elem),
                Float_(elem) => Float_(elem),
                Bool_(elem) => Bool_(elem),
                String_(elem) => String_(elem),
                Variable(elem) => Variable(elem),
                _ => { String_(elem) }
            };
    }
    else {
        new_elem =
            if i.contains('[') { List_(Vec::new()) }
            else if i.contains('{') { Block_(Vec::new()) }
            else { return; };
    }
    vec.push(new_elem);
}

// Removes unnecessary characters
fn trim(i: String) -> String {
    i.trim().trim_matches(|c|
        c == ' ' || c == '"' ||
            c == '[' || c == ']' ||
            c == '{' || c == '}')
        .to_string()
}