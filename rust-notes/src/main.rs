/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */

#[allow(unused_imports)]
use crate::{
    basic::basic, collections::collections, collections::median, collections::mode,
    collections::pig_latin, enums::enums, ownership::ownership, type_handling::type_handling,
};

pub mod basic;
pub mod collections;
pub mod enums;
pub mod error_handling;
pub mod ownership;
pub mod type_handling;

fn main() {
    // Main can be allowed to return Result values, so you can use ? on functions in main()

    // Rust and C both return 0 from their executables if the function processed successfully
    // A non-zero return value represents some kind of error
    type_handling();
}
