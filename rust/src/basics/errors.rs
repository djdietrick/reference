use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

#[allow(dead_code)]
pub fn main() {
    let r = result();
    // Can use match to check for errors
    match r {
        Ok(()) => println!("OK!"),
        Err(()) => println!("Err!"),
    }


}

#[allow(dead_code)]
fn panic() {
    let _v = vec![1, 2, 3];

    _v[99]; // Will cause panic

    panic!("Oh no! Something went wrong!");
}

#[allow(dead_code)]
fn result() -> Result<(),()> {
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    let err = false;
    if err { return Err(()); }


    Ok(())
}

#[allow(dead_code)]
fn error_kind() {
    let _f = File::open("hello.txt");

    let _f = match _f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

#[allow(dead_code)]
fn unwrap_or_else() {
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

#[allow(dead_code)]
fn panic_on_error() {
    // Will panic if open returns Err
    let _f = File::open("hello.txt").unwrap();

    // Panic with custom message
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}

#[allow(dead_code)]
fn propogating() -> Result<String,io::Error> {
    // Will propogate error to calling function
    // Place ? after a Result variable
    // Can only be used in functions that return a Result or Option
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    // Results that have ? called on the go through the 'from' function,
    // implemented in the From trait, so be converted into the error type
    // specified in the return

    // Can also chain them together
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}