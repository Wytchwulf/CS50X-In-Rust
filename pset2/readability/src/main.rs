use std::io;

fn main() {
    // Take user input
    println!("Enter text to be graded");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    // Count Chars
    let letters = input.chars().filter(|c| c.is_alphabetic()).count();

    println!("input = {}letters = {}", input, letters);

    // Count Words

    // Count Sentences

    // Print result
}
