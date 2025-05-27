use clap::Parser;
use std::io::stdin;

const MAX_CANDIDATES: usize = 9;

struct Pair {
    winner: u32,
    loser: u32,
}

#[derive(Parser)]
#[command(name = "Tideman", about = "Tideman Voting System")]
struct Args {
    #[arg(required = true)]
    candidates: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let candidates: Vec<String> = args.candidates;
    let candidate_count: usize = candidates.len();

    if candidate_count > MAX_CANDIDATES {
        println!("Maximum number of candidates is {}", MAX_CANDIDATES);
        return;
    }

    let locked: Vec<Vec<bool>> = vec![vec![false; candidate_count]; candidate_count];

    let voter_count: u32 = prompt(Some("Number of voters: "))
        .parse()
        .expect("Invalid Entry");

    for voter in voter_count {}
}

fn prompt(msg: Option<&str>) -> String {
    if let Some(value) = msg {
        println!("{}", value);
    }
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read line");
    input.trim().to_string()
}
