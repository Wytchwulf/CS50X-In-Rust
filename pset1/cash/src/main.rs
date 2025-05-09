use std::io;

fn main() {
    // Take input from user
    println!("Please enter total");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    // Convert to f64
    let float_val: f64 = input.trim().parse().expect("Could not parse");

    // Convert to cents
    let mut cents: u32 = (float_val * 100.0).round() as u32;

    // Initialise coins list
    let coins = [25, 10, 5, 1];

    // Count the number of times cents is divided by each coin
    let mut count: u32 = 0;
    for coin in coins {
        count += cents / coin;
        cents %= coin;
    }

    println!("{} coins", count);
}

