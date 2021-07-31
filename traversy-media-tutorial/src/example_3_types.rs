// Primitive Types

// Integers:
//   u8, i8, u16, i16, u32, i32, u64, i64, u128, i128

// Floats
//   f32, f64

// Boolean
//   bool (true/false)

// Characters
//   char

// Tuples (different types possible)
//   (1,"hello",12.2)

// Arrays (same type and fixed length)
//   [1,2,3,4]

// Vectors ("extensable array")

pub fn run() {
    println!("=== TYPES ===");

    let x = 12; // infers type i32
    let float = 2.54; // infers type f64
    let y: i64 = 123123123123; // define explicitly
    println!("x = {}\nfloat = {}\ny = {}", x, float, y);

    // display all ranges sizes
    println!("\nINTEGERS");
    println!("  u8: {} to {}", std::u8::MIN, std::u8::MAX);
    println!("  i8: {} to {}", std::i8::MIN, std::i8::MAX);
    println!(" u16: {} to {}", std::u16::MIN, std::u16::MAX);
    println!(" i16: {} to {}", std::i16::MIN, std::i16::MAX);
    println!(" u32: {} to {}", std::u32::MIN, std::u32::MAX);
    println!(" i32: {} to {}", std::i32::MIN, std::i32::MAX);
    println!(" u64: {} to {}", std::u64::MIN, std::u64::MAX);
    println!(" i64: {} to {}", std::i64::MIN, std::i64::MAX);
    println!("u128: {} to {}", std::u128::MIN, std::u128::MAX);
    println!("i128: {} to {}", std::i128::MIN, std::i128::MAX);
    println!("\nFLOATS (❌❌❌ THIS DOES NOT WORK ❌❌❌)");
    println!("f32: {} to {}", std::f32::MIN, std::f32::MAX);
    println!("f64: {} to {}", std::f64::MIN, std::f64::MAX);

    // Boolean
    let is_active: bool = true;
    // get from expression
    let is_greater_than_10 : bool = 2 > 10;
    println!("");
    println!("{:?}", (is_active, x, float, is_greater_than_10));

    // Char
    let a1 = 'a';
    let face = '\u{1f600}'; // unicode (emojis)
    println!("{:?}", (a1, face));
}
