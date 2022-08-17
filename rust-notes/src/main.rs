/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */
use crate::{basic::basic, car::car, collections::collections, enums::enums, ownership::ownership};

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
    collections();
}
