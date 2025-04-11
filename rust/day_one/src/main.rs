use std::io; // Import io for stdin

fn main() {
    // Calculate z from x and y
    let x = 100;
    let y = 200;
    let z = x * y * (x * 3); // x * y * (x * 3) = 100 * 200 * (100 * 3) = 6,000,000

    println!("Z is {:?}", z); // Debug print

    // Get user's name
    println!("Hi, Welcome! What is your name please? 😊");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    let name = name.trim();

    // Get user's age
    println!("Hello, {}! How old are ya?", name);
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read age");
    let age: i32 = age.trim().parse().expect("Expected a number");

    // Print greeting and age
    println!("{}, you're {} years old, nice! 🔥", name, age);

    // Check maturity
    if age >= 18 {
        println!("{}, you are mature!", name);
    } else {
        println!("{}, you are not mature.", name);
    }
}