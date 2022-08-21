/*
Name: Jacob Collins
Description:
Integration tests for rust-notes. Tests the public functions in lib.rs
Tests public interface of the package as though you were a user
Last Edited: August 21, 2022
*/

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, notes::adder::add_two(2));
}
