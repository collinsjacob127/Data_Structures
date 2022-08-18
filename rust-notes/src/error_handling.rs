/* Jacob Collins
 * error_handling.rs
 * practices for handling various errors
 * August 17, 2022
 */

use std::fs::File;
use std::io::ErrorKind;

pub fn error_handling() {
    let greeting_file_result = File::open("hello.txt");

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
