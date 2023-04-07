
use std::io;
use std::io::{Write};

pub fn slay() {

    let mut str: Vec<String> = Vec::new();

    loop {
        print!("bprog> ");
        io::stdout().flush();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let newEl: Vec<&str> = input.split_whitespace().collect();

        println!("Stack: ");
        for i in newEl {
            str.push(i.parse().unwrap());
        }



        for i in str.iter().rev() {
            println!("{}",i);
        }


    }

}