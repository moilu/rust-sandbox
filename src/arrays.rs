// Arrays - Fixed list where elements are the same data types

// If we imported the standard library up here, we dont need to
// right later.
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Print arrays
    println!("{:?}", numbers);

    // Print a value
    println!("{}", numbers[0]);

    // Print array length
    println!("{}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}