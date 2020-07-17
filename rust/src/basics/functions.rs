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

