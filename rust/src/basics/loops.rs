// Loop, do forever
#[allow(dead_code)]
fn infinite_loop() {
    loop {
        println!("To infinity...");
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
// While loop
fn while_loop() {
    let mut x = 0;
    while x < 10 {
        println!("{}", x);
        x+=1;
    }
}

#[allow(dead_code)]
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