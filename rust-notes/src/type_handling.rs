/* Name: Jacob Collins
* Description:
Assessment of generics, types, and lifetimes
* Last Edited: August 18 2022
*/

use crate::type_handling::{generics::generics, lifetimes::lifetimes, traits::traits};
use std::fmt::Display;

mod generics;
mod lifetimes;
mod traits;

pub fn type_handling() {
    generics();
    lifetimes();
    traits();
}

// Generics, traits, and lifetimes all implemented together:
#[allow(dead_code)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
