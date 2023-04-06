

use std::io;

pub fn slay() {

    println!("Please input something");

    let apples = 5;
    let bananas = 10;

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
//    let str: Vec<&str> = input.split_whitespace().collect();

    println!("Apples = {apples}, Bananas = {bananas}");
    println!("Apples + Bananas = {}", apples + bananas);

    for str in input.split_whitespace() {
        println!("Input divided: {}", str);
    }

}