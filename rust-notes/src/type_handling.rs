/* Name: Jacob Collins
* Description:
Assessment of generics, types, and lifetimes
* Last Edited: August 18 2022
*/

use crate::type_handling::{generics::generics, lifetimes::lifetimes, traits::traits};

mod generics;
mod lifetimes;
mod traits;

pub fn type_handling() {
    generics();
    lifetimes();
    traits();
}
