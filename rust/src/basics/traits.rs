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
#[allow(dead_code)]
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

// Associated types
// Types to be used in definitions of the type but specified when implemented
trait HasId {
    type Id;

    fn get_id(&self) -> Option<Self::Id>;
}

impl HasId for Tweet {
    type Id = u32;
    
    fn get_id(&self) -> Option<Self::Id> {
        Some(5)
    }
}

// Disambiguation, when you have functions with the same name
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn _disambiguation() {
    println!("A baby dog is called a {}", Dog::baby_name()); // Spot
    
    // Will not work because we can only call associated functions on implementations of the trait
    //println!("A baby dog is called a {}", Animal::baby_name());

    // Type as Trait>::function(receiver_if_method, next_arg, ...);
    // Received would be the instance of dog if necessary
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // puppy
}

// Super trait
// Trait required for another trait's implementation
use std::fmt;

// OutlinePrint is dependent on Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Newtype pattern
// Usually you can only implement a trait on a type when the crate owns either one
// Can get around this with a Wrapper (tuple struct) and implement the trait on the type
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn _new_type() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}