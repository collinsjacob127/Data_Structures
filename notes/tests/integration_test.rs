/*
Name: Jacob Collins
Description:
Integration tests for rust-notes. Tests the public functions in lib.rs
Last Edited: August 21, 2022
*/

#[test]
fn it_adds_two() {
    assert_eq!(4, notes::adder::add_two(2));
}
