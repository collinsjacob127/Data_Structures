/*
Name: Jacob Collins
Description:
An exploration of how tests can be implemented in rust.
I've already dealt with tests a good amount so will
only be including things that I'm not already familiar with.
Last Edited: August 20 2022
*/

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn rectangle_can_hold() {
        let rect1 = Rectangle {
            width: 7,
            height: 3,
        };
        let rect2 = Rectangle {
            width: 1,
            height: 1,
        };
        let rect3 = Rectangle {
            width: 7,
            height: 3,
        };

        // You can use assert as a format! output for custom error msgs
        assert_ne!(
            rect1, rect2,
            "{:?} and {:?} appear the same size",
            rect1, rect2
        );
        assert_eq!(rect1, rect3);
    }
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

#[allow(dead_code)]
impl Guess {
    pub fn new(value: i32) -> Guess {
        #[allow(clippy::manual_range_contains)]
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    // This is the attribute that tells compiler this test SHOULD panic
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
