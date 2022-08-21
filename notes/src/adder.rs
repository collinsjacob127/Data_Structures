/*
Name: Jacob Collins
Description:
File for use in conjunction with integration_tests.rs and lib.rs
Last Edited: August 21 2022
*/

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// Unit tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
