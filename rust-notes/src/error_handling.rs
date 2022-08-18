/* Jacob Collins
 * error_handling.rs
 * practices for handling various errors
 * August 17, 2022
 */

use std::fs::File;

pub fn error_handling() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => panic!("File read failed: {}", err),
    };
}
