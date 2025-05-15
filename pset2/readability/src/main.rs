use std::io;

fn main() {
    // Take user input
    println!("Enter text to be graded");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    // Analyse input

    // Print result
}
