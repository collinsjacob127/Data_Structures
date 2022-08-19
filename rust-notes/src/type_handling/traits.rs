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
