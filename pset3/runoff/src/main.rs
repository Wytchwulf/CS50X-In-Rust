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
        .expect("Incorrect Input Type");

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

    loop {
        tabulate(&preferences, &mut candidates);

        if print_winner(&candidates) {
            return;
        };

        let min: u32 = find_min(&candidates);

        if is_tie(&candidates, min) {
            for candidate in &candidates {
                if !candidate.eliminated {
                    println!("{}", candidate.name);
                }
            }
            return;
        }

        eliminate(&mut candidates, min);

        for candidate in &mut candidates {
            candidate.votes = 0;
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
        if candidate.name.eq_ignore_ascii_case(name) {
            preferences[voter][rank] = index;
            return true;
        }
    }
    false
}

fn tabulate(preferences: &Vec<Vec<usize>>, candidates: &mut [Candidate]) {
    for voter in preferences {
        for &index in voter {
            if !candidates[index].eliminated {
                candidates[index].votes += 1;
                break;
            }
        }
    }
}

fn print_winner(candidates: &[Candidate]) -> bool {
    let vote_total: u32 = candidates.iter().map(|c| c.votes).sum();

    for candidate in candidates {
        if candidate.votes > vote_total / 2 {
            println!("Winner: {}", candidate.name);
            return true;
        }
    }
    false
}

fn find_min(candidates: &[Candidate]) -> u32 {
    candidates
        .iter()
        .filter(|c| !c.eliminated)
        .map(|c| c.votes)
        .min()
        .unwrap_or(0)
}

fn is_tie(candidates: &[Candidate], min: u32) -> bool {
    for candidate in candidates {
        if !candidate.eliminated && candidate.votes != min {
            return false;
        }
    }
    true
}

fn eliminate(candidates: &mut [Candidate], min: u32) {
    for candidate in candidates {
        if !candidate.eliminated && candidate.votes == min {
            candidate.eliminated = true;
        }
    }
}
