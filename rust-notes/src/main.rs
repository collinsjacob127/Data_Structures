/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */

#[allow(unused_imports)]
use crate::{
    basic::basic, car::car, collections::collections, collections::median, collections::mode,
    collections::pig_latin, enums::enums, error_handling::error_handling, ownership::ownership,
};

pub mod basic;
pub mod car;
pub mod collections;
pub mod enums;
pub mod error_handling;
pub mod ownership;

fn main() {
    // basic();
    // ownership();
    // enums();
    // car();
    //     collections();
    //    let num_list = vec![0, 1, 2, 3, 4, 4, 5];
    //    println!("Mode: {}", mode(&num_list));
    //    println!("Median: {}", median(&num_list));
    //    pig_latin("A very hairy man died today. He was quite thick.");
    error_handling();
}
