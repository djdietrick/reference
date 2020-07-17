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
