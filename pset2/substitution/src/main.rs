use std::collections::HashSet;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Substitution", long_about = "Substitution Based Encryption")]
struct Args {
    #[arg(short)]
    key: String,
} 

fn main() {
    let args = Args::parse();

    if validation(args.key) == false {
        println!("Fuck off");
        return;
    } else {
        println!("Yassssss");
        return;
    };
}

 fn validation(key: String) -> bool {
    let mut repeated = HashSet::new();

    if key.chars().count() != 26 { return false; };
    for char in key.chars() {
        if !char.is_ascii_alphabetic() {
            return false;
        } else if repeated.contains(&char) {
            return false;
        } else {
            repeated.insert(char);
        }
    };

    true
 }