pub fn run() {
    //Print to console
    println!("Hola desde el archivo print.rs");

    // Basic Formatting
    println!("{} es de {}","Moises", "Ensenada");

    //Positional Arguments
    println!("{0} es de {1} y a {0} le gusta {2}", "Moises", "Ensenada", "programar" );

    // Named Arguments
    println!("{name} le gusta {activity}", name = "Moises", activity = "programar");

    // Placeholder traits
    println!("Binario: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}