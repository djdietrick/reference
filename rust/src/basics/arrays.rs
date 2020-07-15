// Arrays in Rust have a fixed length
// Allocate on the stack instead of on the heap
let a = [1, 2, 3, 4, 5];

// Specify type and length
let b: [i32; 5] = [1, 2, 3, 4, 5];

// Create with initial value
let c = [3; 5]; // [3,3,3,3,3]

// Accessing
let one = a[0];
let two = b[1];

// Won't compile because we know the size at compile time
// let n = a[10];

