// Basic void function, no arguments
#[allow(dead_code)]
pub fn basic() {
    println!("Basic test!");
}

// Return value
#[allow(dead_code)]
fn five() -> i32 {
    5
}

#[allow(dead_code)]
fn take_ownership(x: i32) {
    println!("I own {} now", x);
}

#[allow(dead_code)]
fn borrow(x: &i32) {
    println!("I'm just borrowing {}, you can use it again later", x);
}

#[allow(dead_code)]
fn generic<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Fnctions as arguments
#[allow(dead_code)]
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}