// Variables hold a primitive data or reference to data
// Variables are inmutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Moi";
    let mut age = 30;

    println!("Mi nombre es {} y tengo {} de edad", name, age);

    age = 33;

    println!("Mi nombre es {} y tengo {} de edad", name, age);

    // Define a constant
    // When you declare a constant you have to give it a type 
    // in this case the type is i32 which is an 32bit integer.
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age ) = ("Moi", 30);
    println!("{} tiene {}", my_name, my_age);
}