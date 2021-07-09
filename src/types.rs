/*
Tipos primitivos de Rust
Integers: u8, i8, i16, u32, i32, u64, i64, u128, i128 (number of
bit they take in memory.

Floats: f32, f64

Boolean (bool)

Characters (chars)

Tuples

Arrays
*/

//Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how wew use it.

pub fn run() {
    //Deafault is i32
    let x = 1;

    //Default is f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4374623827368;

    //Find max size
    println!("Mx i32: {}", std::i32::MAX);
    println!("Mx i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get a boolean from an expression
    let is_greater: bool = 10 < 5;

    //Char, we use this for a single char. Emojis and other characters from the unicode are of this type.
    let a1 = "a";
    let emoji = "\u{1F600}";

    println!("{:?}", (x, y, z, is_active, is_greater, a1, emoji));

}