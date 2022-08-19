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
    // fn summarize(&self) -> String;

    // method  with default:
    fn summarize(&self) -> String {
        String::from("What an interesting summary I am!")
    }

    // There can be multiple necessary methods per trait
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    // Other methods required by summary if necessary:
    // ...
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
