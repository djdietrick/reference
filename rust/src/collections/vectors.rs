#[allow(dead_code)]
pub fn main() {
    let mut v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];

    v.push(1);
    v.push(2);
    v.push(3);

    // Accessing elements
    // Either with brackets
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    v.push(4);
    // cannot use third after this because the vector has changed

    // Or get
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    if let Some(third) = v.get(2) {
        println!("The third element is {}", third);
    } else {
        println!("There is no third element.");
    }

    // Iterate over vector
    for i in &v {
        println!("{}", i);
    }
    // Iterate over mutable references
    for i in &mut v {
        *i += 50;
    }

    let _last = v.pop(); // 4
}

#[allow(dead_code)]
fn multiple_types() {
    // Using enums to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}