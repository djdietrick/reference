// Basically an interface
// Define functions that will be implemented by structs that have this trait

// Can only implement a trait if either the trait or the type belongs to the local crate

use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary {
    fn summarize(&self) -> String;

    // Can call other methods in the trait
    fn default_method(&self) -> String {
        String::from("Default method, can be overridden")
    }
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

// Using traits as parameters
// Use impl to specify that its a trait
#[allow(dead_code)]
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

#[allow(dead_code)]
pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// Multiple traits
#[allow(dead_code)]
pub fn notify3(_item: &(impl Summary + Display)) {

}
#[allow(dead_code)]
pub fn notify4<T: Summary + Display>(_item: &T) {

}

#[allow(dead_code)]
fn some_function<T, U>(_t: &T, _u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    return 5;
}

// Return items with traits
// Cannot have paths that return different types (cannot return NewsArticle or Tweet)
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}