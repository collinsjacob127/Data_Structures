/* Name: Jacob Collins
* Description:
Notes on traits, their uses, implementations, etc.
* Last Edited: August XX 2022
*/

pub fn traits() {
    todo!();
}

// Orphan rule:
// you can't define a non-local trate (e.g. Display) on a non-local type (e.g. Vec)
pub trait Summary {
    // Method that all types which implement Summary must
    // provide their own definition specific to the type:

    // method as signature:
    // REQUIRES IMPLEMENTATION
    fn summarize_author(&self) -> String;

    // method  with default:
    // This default happens to call the above method
    // Only the above method need be defined
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Overwriting default for custom value:
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    // Other methods required by summary if necessary:
    // ...
}

// Loading default:
// impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
