use std::io;

fn main() {
    // Take user input
    println!("Enter text to be graded");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");

    // Count Chars
    let letters: f64 = input.chars().filter(|c| c.is_alphabetic()).count() as f64;

    // Count Words
    let words: f64 = input.chars().filter(|c| c.is_whitespace()).count() as f64;

    // Count Sentences
    let sentence_enders = ['.', '?', '!'];
    let sentences: f64 = input.chars().filter(|c| sentence_enders.contains(c)).count() as f64;

    // Avg letters in 100 words (L)
    let l: f64 = (letters / words) * 100.0;

    // Avg sentences in 100 words (S)
    let s: f64 = (sentences / words) * 100.0;

    // Get grade
    let grade = 0.0588 * l - 0.296 * s - 15.8;

    if grade.round() < 1.0 {
        println!("Before Grade 1")
    } else if grade.round() > 16.0 {
        println!("Grade 16+");
    } else {
        println!("Grade {}", grade.round());
    }
}
