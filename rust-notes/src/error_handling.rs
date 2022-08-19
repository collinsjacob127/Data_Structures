/* Jacob Collins
 * error_handling.rs
 * practices for handling various errors
 * File IO
 * Exe IO
 * August 17, 2022
 */

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

pub fn match_error_handling() {
    let greeting_file_result = File::open("hello.txt");

    #[allow(unused_variables)]
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error with file create: {}", e),
            },
            other_error => {
                panic!("Error with file open: {}", other_error);
            }
        },
    };
}

pub fn expect_error_handling() {
    // expect is just better unwrap
    // let greeting_file = File::open("hello.txt").unwrap(); // doesn't include an error message
    #[allow(unused_variables)]
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

#[allow(dead_code)]
fn match_read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

#[allow(dead_code)]
fn qmark_read_username_from_file() -> Result<String, io::Error> {
    // ? can only be used on functions of the returned type; type must be a Return or Option
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

#[allow(dead_code)]
fn fs_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
