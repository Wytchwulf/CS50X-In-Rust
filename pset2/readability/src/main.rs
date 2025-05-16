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

    // Count Words
    let words = input.chars().filter(|c| c.is_whitespace()).count();

    println!("input = {}letters ={}\nwords = {}", input, letters, words)

    // Count Sentences

    // Print result
}
