// Loop, do forever
fn infinite_loop() {
    loop {
        println("To infinity...");
    }
}

// Returning from loops
fn return_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

// While loop
fn while_loop() {
    let x = 0;
    while x < 10 {
        println!("{}", x);
        x++;
    }
}

// For loop
fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}