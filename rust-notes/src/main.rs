/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */

#[allow(unused_imports)]
use crate::{
    basic::basic,
    car::car,
    collections::collections,
    collections::median,
    collections::mode,
    collections::pig_latin,
    enums::enums,
    error_handling::match_error_handling,
    generics_types::{generics_types, mixup},
    ownership::ownership,
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

    mixup();
}
