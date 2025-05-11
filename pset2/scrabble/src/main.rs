fn main() {
    println!("Player One, Enter your word:");
    let p_one = get_word();

    println!("Player Two, Enter your word:");
    let p_two = get_word();

    let p_one_score = get_score(&p_one);
    let p_two_score = get_score(&p_two);

    if p_one_score > p_two_score {
        println!("Player One Wins");
    } else {
        println!("Player Two Wins");
    }
}

fn get_word() -> String {
    let mut word = String::new();
    std::io::stdin()
        .read_line(&mut word)
        .expect("Could not read line");
    word.trim().to_string()
}

fn get_score(word: &str) -> u32 {
    let points = [
        1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10,
    ];

    word.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| points[(c as u8 - b'a') as usize])
        .sum()
}
