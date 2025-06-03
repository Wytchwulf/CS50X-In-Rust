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

struct Tideman {
    candidates: Vec<String>,
    preferences: Vec<Vec<u32>>,
    locked: Vec<Vec<bool>>,
    pairs: Vec<Pair>,
}

impl Tideman {
    fn new(candidates: Vec<String>) -> Self {
        let count = candidates.len();
        Self {
            candidates,
            preferences: vec![vec![0; count]; count],
            locked: vec![vec![false; count]; count],
            pairs: Vec::new(),
        }
    }

    fn vote(&mut self, rank: usize, name: &str, ranks: &mut [usize]) -> bool {
        if let Some(index) = self
            .candidates
            .iter()
            .position(|candidate| candidate == name)
        {
            ranks[rank] = index;
            true
        } else {
            false
        }
    }

    fn record_preferences(&mut self, ranks: &[usize]) {
        for i in 0..ranks.len() {
            for j in (i + 1)..ranks.len() {
                self.preferences[ranks[i]][ranks[j]] += 1;
            }
        }
    }

    fn add_pairs(&mut self) {
        todo!();
    }

    fn sort_pairs(&mut self) {
        todo!();
    }

    fn lock_pairs(&mut self) {
        todo!();
    }

    fn print_winner(&self) {
        todo!();
    }
}

fn main() {
    let args = Args::parse();
    let candidates: Vec<String> = args.candidates;

    if candidates.len() > MAX_CANDIDATES {
        println!("Maximum number of candidates is {}", MAX_CANDIDATES);
        return;
    }

    let mut tideman = Tideman::new(candidates);
    let voter_count: usize = prompt("Number of voters: ").parse().expect("Invalid Entry");

    for _ in 0..voter_count {
        let mut ranks = vec![0; tideman.candidates.len()];
        for (j, _) in tideman.candidates.iter().enumerate() {
            let name = prompt(&format!("Rank {}:", j + 1));
            if !tideman.vote(j, &name, &mut ranks) {
                println!("Invalid Vote");
                return;
            }
        }
        tideman.record_preferences(&ranks);
        println!();
    }

    tideman.add_pairs();
    tideman.sort_pairs();
    tideman.lock_pairs();
    tideman.print_winner();
}

fn prompt(msg: &str) -> String {
    println!("{}", msg);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read line");
    input.trim().to_string()
}
