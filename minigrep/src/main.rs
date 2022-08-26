/*
Name: Jacob Collins
Description:
Making a program to implement a basic grep procedure in rust.
- Command line arguments
Last Edited: August 23 2022
*/
use std::env;
use std::fs;

/*
Guidelines for writing binary projects in rust:
Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
As long as your command line parsing logic is small, it can remain in main.rs.
When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
*/

fn main() {
    // Use std::env::args_os if you need to collect invalid unicode characters from the command
    // Parse cmd line arguments
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // Run the program and handle errors

    println!("Searching for file: {}", file_path);
    println!("Searching for query: {}", query);

    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("The file should have been accessible");

    println!("Contents:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}
