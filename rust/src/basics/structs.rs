#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    age: i32,
    active: bool
}
// Note, use String instead of &str in structs because we want the data 
// to live on the structs stack and die when the struct does

#[allow(dead_code)]
pub fn create_users() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        age: 29,
    }; 
    
    // ..user1 copies over all other fields
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        ..user1
    };
    
    user2.email = String::from("anotheremail@example.com");
}

#[allow(dead_code)]
fn tuple_structs() { 
    // Tuple structs
    struct Color(i8,i8,i8);
    let _black = Color(0, 0, 0);
}


// Unit like struct
#[allow(dead_code)]
struct Unit {}

// To be able to print structs, you should use the Debug trait
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

#[allow(dead_code)]
fn print_debug() {
    let rect = Rectangle {
        width: 3,
        height: 5
    };
    
    println!("My rect is {:?}", rect);
}

// Implementing methods of structs
impl Rectangle {
    // Associated function because it doesnt take self
    #[allow(dead_code)]
    fn new(w: i32, h: i32) -> Rectangle {
        Rectangle {
            width: w,
            height: h
        }
    }
    #[allow(dead_code)]
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

#[allow(dead_code)]
fn generics() {
    
    // Generics in structs
    struct Point<T> {
        x: T,
        y: T
    }

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
}
