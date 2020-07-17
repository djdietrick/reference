#[allow(dead_code)]
pub fn main() {
    // Tuples have a fixed length, once declared you can't change
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (_x,_y,_z) = tup;

    //Accessing
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
}

