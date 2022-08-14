/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */
use crate::basic::basic;
use crate::enums::enums;
use crate::ownership::ownership;
use crate::structs::structs;
// Example of module scope
use crate::car::body;
use crate::car::engine;

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
    foo_pub();
}
