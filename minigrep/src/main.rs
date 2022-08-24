/*
Name: Jacob Collins
Description:
Making a program to implement a basic grep procedure in rust.
- Command line arguments
Last Edited: August 23 2022
*/
use std::env;

fn main() {
    // Use std::env::args_os if you need to collect invalid unicode characters from the command
    // line arguments supplied
    let args: Vec<String> = env::args().collect();

    let query = &args[1]; // The string we're searching for
    let filepath = &args[2];

    println!("Searching for file: {}", filepath);
    println!("Searching for query: {}", query);
}
