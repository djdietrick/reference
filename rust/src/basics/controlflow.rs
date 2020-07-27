#[allow(dead_code)]
pub fn main() {
    // If, elseif else
    let num = 5;
    if num < 3 {
        println!("Less than 3");
    } else if num <= 3 && num < 7 {
        println!("In the sweet spot");
    } else {
        println!("Greater!");
    }

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // Conditional setting
    let condition = false;
    let _number = if condition { 5 } else { 6 };

    // Match
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    fn placeholder(x: i32) {
        match x {
            1 => print!("One"),
            2 => print!("Two"),
            3 => print!("Three"),
            _ => print!("Other!"),
        }
    }

    // Or
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Range
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // If let
    // Used for options
    #[allow(irrefutable_let_patterns)]
    if let p1 = plus_one(Some(1)) {
        println!("{:?}", p1);
    } else {
        println!("None!");
    }
}
