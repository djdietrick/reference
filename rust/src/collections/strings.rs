#[allow(dead_code)]
pub fn main() {

    // String literal, immutable and stored on the stack (because size must be known at compile time)
    // Uses double quotes as opposed to char's single quotes
    let _lit = "hello world";

    // Basic string, mutable and stored on the heap (because size can be unknown at compile time)
    let mut s = String::from("hello");

    s.push_str(", world"); // push_str() appends a literal to a String
    s.push('!'); // Pushes single character

    println!("{}", s); // This will print `hello, world!`

    let mut _empty = String::new();

    // + to combine 2 strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

#[allow(dead_code)] 
fn format() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);
}

#[allow(dead_code)]
fn slicing() {
    let hello = "Здравствуйте";

    let _s = &hello[0..4]; // Contains first 4 bytes, or 2 letters
}

#[allow(dead_code)]
fn iterating() {
    for c in "Hello".chars() {
        println!("{}", c);
    }
    for b in "Hello".bytes() {
        println!("{}", b);
    }
}