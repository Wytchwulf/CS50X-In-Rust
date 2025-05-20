use clap::Parser;
use std::io;

const MAX: usize = 9;

struct Candidate {
    name: String,
    votes: u32,
}

#[derive(Parser)]
#[command(name = "Plurality", about = "Plurality Voting System")]
struct Args {
    candidates: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.candidates.len() > MAX {
        println!("Max number of candidates reached");
        std::process::exit(1);
    };

    let mut candidates: Vec<Candidate> = args
        .candidates
        .into_iter()
        .map(|name| Candidate { name, votes: 0 })
        .collect();

    let voter_count: u32 = prompt("Number of voters: ").parse().expect("Invalid Entry");

    for _ in 0..voter_count {
        let name = prompt("Vote: ");
        if !record_vote(&mut candidates, &name) {
            println!("Invalid Vote");
        }
    }

    print_winner(&candidates);
}

fn prompt(msg: &str) -> String {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input");
    input.trim().to_string()
}

fn record_vote(candidates: &mut [Candidate], name: &str) -> bool {
    for candidate in candidates.iter_mut() {
        if candidate.name.to_lowercase() == name.to_lowercase() {
            candidate.votes += 1;
            return true;
        }
    }
    false
}

fn print_winner(candidates: &[Candidate]) {
    let max_votes = candidates.iter().map(|c| c.votes).max().unwrap_or(0);
    for candidate in candidates {
        if candidate.votes == max_votes {
            println!("The winner is {}", candidate.name);
        }
    }
}
