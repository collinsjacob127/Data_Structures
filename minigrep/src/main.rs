/*
Name: Jacob Collins
Description:
Making a program to implement a basic grep procedure in rust.
- Command line arguments
Last Edited: August 23 2022
*/
use minigrep::config::Config;
use std::{env, process};

/*
Guidelines for writing binary projects in rust:
Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
As long as your command line parsing logic is small, it can remain in main.rs.
When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
*/

fn main() {
    // Parse cmd line arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Run the program and handle errors
    if let Err(e) = minigrep::run(config) {
        eprintln!("Problem reading file: {e}");
        process::exit(1);
    };
}
