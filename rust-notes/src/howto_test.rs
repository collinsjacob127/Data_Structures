/*
Name: Jacob Collins
Description:
An exploration of how tests can be implemented in rust.
I've already dealt with tests a good amount so will
only be including things that I'm not already familiar with.
Last Edited: August 20 2022
*/

// Cargo commands for test:
// cargo test -x -y -z -- -a -b -c
// before the '--': arguments relating to the tests
// after the '--': arguments relating to the binary

// How to switch from parallel testing to series:
// $ cargo test -- --test-threads=1
//
// ^ That is useful if the tests would interfere with eachother
// e.g. file i/o with the same file or global variables etc.

// How to show output:
// $ cargo test -- --show-output

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
mod error_msgs {
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
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

// Testing with specific error codes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn less_than_1() {
        Guess::new(0);
    }
}

// You can only test against results where the return type on success is null
// #[allow(dead_code)]
// fn base_2_log_possible(x: u32) -> Result<bool, String> {
//     if x % 2 == 0 {
//         Ok(true)
//     } else {
//         Err(String::from("This number can't have an integer result!"))
//     }
//}
//
// // Testing against Result<V, T> (bool)
// #[cfg(test)]
// mod returns1 {
//    use super::*;
//
//     #[test]
//     fn checking_2_log() -> Result<bool, String> {
//         base_2_log_possible(8)
//     }
// }

// Testing against result (empty result)
// lets you just call functions that use result instead of having to manually
// handle potential errors and environments here
#[allow(dead_code)]
fn it_works() -> Result<(), String> {
    #[allow(clippy::eq_op)]
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}

// Testing against Result<V, T>
#[cfg(test)]
mod returns2 {
    use super::*;

    #[test]
    fn checking_2_log() -> Result<(), String> {
        it_works()
    }
}

// Filtering out a time-consuming test unless explicitly run:
// runnning the ignored test:
// $ cargo test -- --ignored
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
