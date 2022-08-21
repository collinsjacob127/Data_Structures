/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */

pub mod basic;
pub mod collections;
pub mod enums;
pub mod error_handling;
pub mod howto_test;
pub mod ownership;
pub mod type_handling;

use notes::adder::add_two;

fn main() {
    // Main can be allowed to return Result values, so you can use ? on functions in main()

    // Rust and C both return 0 from their executables if the function processed successfully
    // A non-zero return value represents some kind of error
    let input = 3;
    println!("{} + 2: {}", input, add_two(input));
}
