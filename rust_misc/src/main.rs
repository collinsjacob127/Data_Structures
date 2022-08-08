/* Jacob Collins
 * main.rs
 * Combines notes etc for output and testing
 */
mod basic;
use crate::basic::basic;
mod ownership;
use crate::ownership::ownership;

fn main() {
    basic();
    ownership();
}
