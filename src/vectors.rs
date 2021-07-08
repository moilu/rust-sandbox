// Vectors are resizeble arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    numbers[2] = 20;

    // Pop off last value
    numbers.pop();

    // Add on to vector
    numbers.push(6);
    numbers.push(7);

    // Print vectors
    println!("{:?}", numbers);

    // Print a value
    println!("{}", numbers[0]);

    // Print Vector length
    println!("{}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Number Vec: {:?}", numbers);
}