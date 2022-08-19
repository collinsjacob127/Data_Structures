/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */

use collections::grouped_hash_interface;

#[allow(unused_imports)]
use crate::{
    basic::basic, car::car, collections::collections, collections::median, collections::mode,
    collections::pig_latin, enums::enums, error_handling::match_error_handling,
    generics_types::generics_types, ownership::ownership,
};

pub mod basic;
pub mod car;
pub mod collections;
pub mod enums;
pub mod error_handling;
pub mod generics_types;
pub mod ownership;

fn main() {
    // Main can be allowed to return Result values, so you can use ? on functions in main()

    // Rust and C both return 0 from their executables if the function processed successfully
    // A non-zero return value represents some kind of error

    // basic();
    // ownership();
    // enums();
    // car();
    //     collections();
    //    let num_list = vec![0, 1, 2, 3, 4, 4, 5];
    //    println!("Mode: {}", mode(&num_list));
    //    println!("Median: {}", median(&num_list));
    //    pig_latin("A very hairy man died today. He was quite thick.");
    grouped_hash_interface();
}
