use clap::Parser;
use std::collections::HashSet;
use std::io;

#[derive(Parser)]
#[command(name = "Substitution", long_about = "Substitution Based Encryption")]
struct Args {
    #[arg(short)]
    key: String,
}

fn main() {
    let args = Args::parse();

    if validation(&args.key) == false {
        println!("Key must be:\n- 26 characters\n- Alphabetic\n- No repeated characters");
        return;
    };

    println!("Enter plaintext to encrypt");

    let mut plaintext = String::new();

    io::stdin()
        .read_line(&mut plaintext)
        .expect("Unable to read input");

    let plaintext = plaintext.trim_end();
    let ciphertext = encrypt(&args.key, &plaintext);

    println!("Plaintext:  {}", plaintext);
    println!("Ciphertext:  {}", ciphertext);
}

fn validation(key: &str) -> bool {
    let mut repeated = HashSet::new();

    if key.chars().count() != 26 {
        return false;
    }
    for char in key.chars() {
        if !char.is_ascii_alphabetic() {
            return false;
        } else if repeated.contains(&char) {
            return false;
        } else {
            repeated.insert(char);
        }
    }

    true
}

fn encrypt(key: &str, plaintext: &str) -> String {
    let mut ciphertext = String::new();

    for char in plaintext.chars() {
        if char.is_ascii_uppercase() {
            let i = char as u8 - b'A';
            let ch = key.as_bytes()[i as usize] as char;
            ciphertext.push(ch.to_ascii_uppercase());
        } else if char.is_ascii_lowercase() {
            let i = char as u8 - b'a';
            let ch = key.as_bytes()[i as usize] as char;
            ciphertext.push(ch.to_ascii_lowercase());
        } else {
            ciphertext.push(char);
        }
    }

    ciphertext
}
