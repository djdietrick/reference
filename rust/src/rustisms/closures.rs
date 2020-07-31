// Like lambdas in other languages, anonymous functions that can be stored as variables and used as function arguments
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }

    // Can also explicitly state the types, but is not necessary
    let _expensive_closure2 = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}

#[allow(dead_code)]
fn types_must_be_same() {
    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));

    // Now can only call example_closure on strings
    
    // Won't work
    // let n = example_closure(5);
}

// Use a cacher pattern to avoid calling the function multiple times for the same value
// This cacher can only hold a single value however. To make it generic for multiple inputs,
// we can use a hashmap instead.
#[allow(dead_code)]
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    #[allow(dead_code)]
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    #[allow(dead_code)]
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// Function types:
// FnOnce - Takes ownership of things captures in its scope and can only be called once
// FnMut - Mutably borrows and captures variables
// Fn - Immutably borrows

#[allow(dead_code)]
fn move_value() {
    let x = vec![1, 2, 3];

    // move signifies that the closure is an FnOnce
    let equal_to_x = move |z| z == x;

    // Can't use x not because we moved it into the closure
    //println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

// Return closures from functions
// Must return a box/ptr because size isn't known at compile time
fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}