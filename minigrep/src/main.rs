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
    let config = Config::new(&args);

    // Run the program and handle errors

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("The file should have been accessible");

    println!("Contents:\n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!();
        }
        Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        }
    }
}
