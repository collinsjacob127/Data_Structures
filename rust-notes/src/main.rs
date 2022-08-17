/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */
use crate::{
    basic::basic, car::car, collections::collections, collections::median, collections::mode,
    enums::enums, ownership::ownership,
};

pub mod basic;
pub mod car;
pub mod collections;
pub mod enums;
pub mod ownership;

fn main() {
    // basic();
    // ownership();
    // enums();
    // car();
    //     collections();
    let num_list = vec![0, 1, 2, 3, 4, 4, 5];
    println!("Mode: {}", mode(&num_list));
    println!("Median: {}", median(&num_list));
}
