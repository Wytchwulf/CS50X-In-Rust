use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Caesar", long_about = "Enter plaintext to encrypt and key")]
struct Args {
    #[arg(short)]
    plaintext: String,

    #[arg(short)]
    key: u8,
}

fn main() {
    let args = Args::parse();

    let mut ciphertext = String::new();

    for char in args.plaintext.chars() {
        if char.is_ascii_uppercase() {
            ciphertext.push(((char as u8 - b'A' + args.key) % 26 + b'A') as char);
        } else if char.is_ascii_lowercase() {
            ciphertext.push(((char as u8 - b'a' + args.key) % 26 + b'a') as char);
        } else {
            ciphertext.push(char);
        }
    }

    println!("Plaintext:  {}", args.plaintext);
    println!("Ciphertext:  {}", ciphertext);
}
