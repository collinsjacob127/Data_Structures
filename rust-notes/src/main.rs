/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */
use crate::basic::basic;
use crate::car::car;
use crate::enums::enums;
use crate::ownership::ownership;
use crate::structs::structs;

pub mod basic;
pub mod car;
pub mod enums;
pub mod ownership;
pub mod structs;

fn main() {
    basic();
    ownership();
    structs();
    enums();
    car();
}
