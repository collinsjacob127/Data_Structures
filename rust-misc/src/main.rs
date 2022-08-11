/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */
mod basic;
use crate::basic::basic;
mod ownership;
use crate::ownership::ownership;
mod structs;
use crate::structs::structs;

fn main() {
    basic();
    ownership();
    structs();
}
