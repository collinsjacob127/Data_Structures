/* Name: Jacob Collins
* Description:
Notes on traits, their uses, implementations, etc.
* Last Edited: August XX 2022
*/

pub fn traits() {
    todo!();
}

pub trait Summary {
    // Method that all types which implement Summary must
    // provide their own definition specific to the type:
    fn summarize(&self) -> String;

    // There can be multiple necessary methods per trait
    fn as_string(&self) -> str;
}
