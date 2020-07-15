// Tuples have a fixed length, once declared you can't change
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Destructuring
let (x,y,z) = tup;

//Accessing
let five_hundred = tup.0;
let six_point_four = tup.1;

