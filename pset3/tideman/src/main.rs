use clap::Parser;
use std::io::stdin;

const MAX_CANDIDATES: usize = 9;

struct Pair {
    winner: usize,
    loser: usize,
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
        for i in 0..self.candidates.len() {
            for j in (i + 1)..self.candidates.len() {
                let i_over_j = self.preferences[i][j];
                let j_over_i = self.preferences[j][i];

                if i_over_j > j_over_i {
                    self.pairs.push(Pair {
                        winner: i,
                        loser: j,
                    });
                } else if j_over_i > i_over_j {
                    self.pairs.push(Pair {
                        winner: j,
                        loser: i,
                    });
                }
            }
        }
    }

    fn sort_pairs(&mut self) {
        self.pairs.sort_by(|a, b| {
            let stren_a = self.preferences[a.winner][a.loser]
                .saturating_sub(self.preferences[a.loser][a.winner]);
            let stren_b = self.preferences[b.winner][b.loser]
                .saturating_sub(self.preferences[b.loser][b.winner]);
            stren_b.cmp(&stren_a)
        });
    }

    fn lock_pairs(&mut self) {
        for pair in &self.pairs {
            if !self.has_cycle(pair.winner, pair.loser) {
                self.locked[pair.winner][pair.loser] = true;
            }
        }
    }

    fn has_cycle(&self, start: usize, current: usize) -> bool {
        if current == start {
            return true;
        }

        for next in 0..self.candidates.len() {
            if self.locked[current][next] {
                if self.has_cycle(start, next) {
                    return true;
                }
            }
        }

        false
    }

    fn print_winner(&self) {
        for col in 0..self.candidates.len() {
            let mut is_source = true;
            for row in 0..self.candidates.len() {
                if self.locked[row][col] {
                    is_source = false;
                    break;
                }
            }

            if is_source {
                println!("Winner: {}", self.candidates[col]);
                return;
            }
        }
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
        for j in 0..tideman.candidates.len() {
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
