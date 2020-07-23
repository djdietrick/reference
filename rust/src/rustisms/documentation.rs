// Use three slashes to indicate a rustdoc comment
// Can include headers with #
// Can implement code between 2 sets of 3 ticks and even run tests

// Run cargo doc to generate HTML documentation
// cargo doc --open opens it in a web browser

// Common sections include Examples, Panic, Errors, and Safety


/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}