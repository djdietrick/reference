use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex,Arc};

#[allow(dead_code)]
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Waits for the threads to finish
    // Otherwise if the main thread finishes first, the other threads automatically die
    // Also blocks the main thread from continuing
    handle.join().unwrap();
}

// Move takes ownership of things in the main thread so it can be used in the spawned thread
fn _move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // Cannot use v after it has been moved

    handle.join().unwrap();
}

#[allow(dead_code)]
fn channel() {
    // MPSC = Multiple producers, single consumer
    // Can have multiple threads sending messages, but only one receiving
    // Channel closes when one end goes out of scope

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // send transfers ownership to the receiver
    });

    // Two methods: recv and try_recv
    // recv blocks until a value is sent down the channel or channel closes
    // try_recv doesn't block and instead returns a Result, so this is useful for loops or to continue on the main thread
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

#[allow(dead_code)]
fn channel_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

#[allow(dead_code)]
fn channel_multiple_transmitters() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

#[allow(dead_code)]
fn mutexes() {
    // Arc is a thread safe Rc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Clone the pointer to counter
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}