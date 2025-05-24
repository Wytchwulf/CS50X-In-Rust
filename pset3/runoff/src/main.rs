use clap::Parser;
use std::io::stdin;

const MAX_VOTERS: usize = 100;
const MAX_CANDIDATES: usize = 9;

struct Candidate {
    name: String,
    votes: u32,
    eliminated: bool,
}

#[derive(Parser)]
#[command(name = "Runoff", about = "Elimination Based Voting System")]
struct Args {
    candidates: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let candidate_count: usize = args.candidates.len();

    if candidate_count > MAX_CANDIDATES {
        println!("Maximum number of candidates is {}", MAX_CANDIDATES);
        return;
    };

    let mut candidates: Vec<Candidate> = args
        .candidates
        .into_iter()
        .map(|name| Candidate {
            name,
            votes: 0,
            eliminated: false,
        })
        .collect();

    let voter_count: usize = prompt(Some("Number of voters: "))
        .parse()
        .expect("Incorrect Input Type: u32");

    if voter_count > MAX_VOTERS {
        println!("Maximum number of voters is {}", MAX_VOTERS);
        return;
    };

    let mut preferences: Vec<Vec<usize>> = vec![vec![0; candidate_count]; voter_count];

    for i in 0..voter_count {
        println!("Voter {}:", i + 1);
        for j in 0..candidate_count {
            loop {
                println!("Rank {}: ", j + 1);
                let name = prompt(None);
                if vote(&name, i, j, &mut preferences, &mut candidates) {
                    break;
                } else {
                    println!("Invalid Vote");
                }
            }
        }
    }
}

fn prompt(msg: Option<&str>) -> String {
    if let Some(value) = msg {
        println!("{}", value);
    };
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read line");
    input.trim().to_string()
}

fn vote(
    name: &str,
    voter: usize,
    rank: usize,
    preferences: &mut Vec<Vec<usize>>,
    candidates: &[Candidate],
) -> bool {
    for (index, candidate) in candidates.iter().enumerate() {
        if candidate.name.to_lowercase() == name.to_lowercase() {
            preferences[voter][rank] = index;
            return true;
        }
    }
    false
}
