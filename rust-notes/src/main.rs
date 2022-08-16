/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */
use crate::{
    basic::basic, car::car, collections::collections, enums::enums, ownership::ownership,
    structs::structs,
};

pub mod basic;
pub mod car;
pub mod collections;
pub mod enums;
pub mod ownership;
pub mod structs;

fn main() {
    basic();
    ownership();
    structs();
    enums();
    car();
    collections();
}
