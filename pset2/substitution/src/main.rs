use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Substitution", long_about = "Substitution Based Encryption")]
struct Args {
    #[arg(short)]
    key: String,
} 

fn main() {
    let args = Args::parse();

    if args.key.chars().count() < 26 {
        println!("Key must be exactly 26 characters");
        return;
    };

    println!("{} chars exactly. Well done", args.key.chars().count());
}
