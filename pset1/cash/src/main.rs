use std::io;

fn main() {
    // Take input from user
    let float_val: f64 = loop {
        println!("Please enter total");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        // Convert to f64
        match input.trim().parse() {
            Ok(num) => if num >= 0.0 {
                break num;
            } else {
                println!("Please enter a positive value");
            }

            Err(_) => { println!("Please enter a valid positive number"); }
        }
    };

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

