// Lifetime is how long a variable lives
// You only have to worry about lifetimes for references

// Static lifetimes are good for the whole life of the program
#[allow(dead_code)]
fn returns_str() -> &'static str {
    let _my_string = String::from("I am a string");
    "I am a str"
}

// This guarantees that name will be valid as long as the City is valid
// The compiler will complain if city_names dies before my_city does
#[derive(Debug)]
struct City<'a> { // City has lifetime 'a
    name: &'a str, // and name also has lifetime 'a.
    date_founded: u32,
}

#[allow(dead_code)]
fn main() {
    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];

    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
}