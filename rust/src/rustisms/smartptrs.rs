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