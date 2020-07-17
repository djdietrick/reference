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

    // If let
    // Used for options
    #[allow(irrefutable_let_patterns)]
    if let p1 = plus_one(Some(1)) {
        println!("{:?}", p1);
    } else {
        println!("None!");
    }
}
