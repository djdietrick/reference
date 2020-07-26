use std::ops::Deref;

// Boxes are stored on the stack but point to data stored on the heap
// Useful for storing recursive types or other things that the size is unknown at compile time

#[allow(dead_code)]
fn basic_box() {
    let b = Box::new(5);
    println!("b = {}", b);

    // Need to dereference with * to access value
    assert_eq!(5, *b);
}
// Both the box and the data it points to gets deallocated when it goes out of scope

// Defining our own smart pointers
// Just a tuple struct with one element
#[allow(dead_code)]
struct MyBox<T>(T);

#[allow(dead_code)]
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Need to implement the Deref trait in order to dereference custom smart ptrs
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// Drop trait is run whenever something goes out of scope
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[allow(dead_code)]
fn drop_test() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

// Reference counted pointer Rc<T>
// Allows for multiple owners of the same data
// Deallocates when the last pointer is deallocated
// Immutable references (reading only)

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     let b = Cons(3, Rc::clone(&a));
//     let c = Cons(4, Rc::clone(&a));
// }

// RefCell<T> allows mutable borrows checked at runtime, you can mutate 
// the value inside the RefCell<T> even when the RefCell<T> is immutable.
// borrow returns Ref<T> while borrow_mut returns a RefMut<T>.
// Can only have one RefMut for a value at a time
// RefCells also keep reference counts for Ref<T>

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}