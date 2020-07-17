use std::collections::HashMap;

// type HashMap<K,V>

#[allow(dead_code)]
pub fn main() {
    let mut scores: HashMap<String,i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing
    let team_name = String::from("Blue");
    let _score: Option<&i32> = scores.get(&team_name);

    // Iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // To Overwrite, just insert with the same key

    // Only inserting if there is no key
    scores.entry(String::from("Blue")).or_insert(50); // Won't overwrite
    scores.entry(String::from("Red")).or_insert(20);

}

#[allow(dead_code)]
fn count() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}