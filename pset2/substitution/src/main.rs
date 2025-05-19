use std::collections::HashSet;
use std::io;
use clap::Parser;

#[derive(Parser)]
#[command(name = "Substitution", long_about = "Substitution Based Encryption")]
struct Args {
    #[arg(short)]
    key: String,
}

fn main() {
    let args = Args::parse();

    if validation(args.key) == false {
        println!("Key must be:\n- 26 charaters\n- Alphabetic\n- No repeated characters");
        return;
    };

    println!("Enter plaintext to encrypt");

    let mut plaintext = String::new();

    io::stdin()
        .read_line(&mut plaintext)
        .expect("Unable to read input");


}

fn validation(key: String) -> bool {
    let mut repeated = HashSet::new();

    if key.chars().count() != 26 {
        return false;
    };
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
fn encrypt(key: String, plaintext: String) -> String {
    
}
