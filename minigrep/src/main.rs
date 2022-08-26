/*
Name: Jacob Collins
Description:
Making a program to implement a basic grep procedure in rust.
- Command line arguments
Last Edited: August 23 2022
*/
use std::{env, error::Error, fs, process};

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
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Run the program and handle errors
    if let Err(e) = run(config) {
        println!("Problem reading file: {e}");
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    println!("Contents:\n{}", contents);
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check that there are enough command line args
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}
