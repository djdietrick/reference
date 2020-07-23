#[allow(dead_code)]
pub fn main() {
    // Scalar Types (or basic types)

    // Integer, either signed or unsigned, bits in (8,16,32,64,128)
    let _num: i32 = -5; 
    let _unsigned_num: u32 = 5;

    // Floats, either 32 or 64 bit
    let _f = 5.0; // Defaults to f64
    let _f2: f32 = 3.9;

    // Boolean
    let _b: bool = true;

    // Char, specified by single-quotes
    let _c: char = 'c';

    // Options
    // enum Option<T> {
    //      Some(T),
    //      None    
    // }
    let _absent_option: Option<i32> = None;
    let _present_option: Option<i32> = Some(5);
}

#[allow(dead_code)]
fn slice() {
    // Slice
    let s = String::from("hello world");

    // String slice &str
    let _hello: &str = &s[0..5];
    let _world = &s[6..11];
    let _world2 = &s[6..];
    let _whole = &s[..];

    // Works with both &str and String
    fn first_word(s: &str) -> &str {
        s
    }

    // Can also slice arrays
    let a = [1, 2, 3, 4, 5];

    let _slice = &a[1..3];
}
