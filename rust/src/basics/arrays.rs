// Arrays in Rust have a fixed length
// Allocate on the stack instead of on the heap
#[allow(dead_code)]
pub fn main() {
    let a = [1, 2, 3, 4, 5];

    // Specify type and length
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // Create with initial value
    let _c = [3; 5]; // [3,3,3,3,3]

    // Accessing
    let _one = a[0];
    let _two = b[1];

    // Won't compile because we know the size at compile time
    // let n = a[10];
}
