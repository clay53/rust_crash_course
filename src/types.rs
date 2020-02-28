/*
Primitives
    Ints: u(unsigned)/i(signed) + number of bits (16, 32, 128) ex: (i32)
    Floats: f32, f64
    Boolean: bool
    Characters: char
    Tuples (like lists)
    Arrays (fixed length)
*/

// Types are static but are inferred by complier

pub fn run() {
    // Default int is i32
    let x = 1;

    // Default is f64
    let y = 2.5;

    // Explicit type
    let z: i64 = 1789234987;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    // Characters
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}