#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(dead_code)]
pub fn greeting(_name: &str) -> String {
    String::from("Hello!")
}

// Unit tests
// These tests are done in the same file as the code they are testing in a module tests
// #[cfg(test)] means this code only compiles when building tests
#[cfg(test)]
mod tests {
    // Brings in code from outside the tests mod
    use super::*;

    #[test]
    fn it_works() {
        // structs that you assert on must implement Debug and PartialEq
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    // Custom assert message
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn expect_panic() {
        panic!("See I should panic");
    }

    #[test]
    #[should_panic(expected = "Expected text")]
    fn expect_panic_message() {
        panic!("Expected text");
    }

    // Return a result to allow for the ? operator to be used to cause test failures
    #[test]
    fn result_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // Ignore test
    // Can still be run on demand
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}