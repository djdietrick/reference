// String literal, immutable and stored on the stack (because size must be known at compile time)
// Uses double quotes as opposed to char's single quotes
let lit = "hello world";

// Basic string, mutable and stored on the heap (because size can be unknown at compile time)
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`

