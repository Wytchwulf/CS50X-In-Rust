use std::io;

fn main() {
    // Take input from user
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    // Convert to int
    let long_no: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Could not convert to integer");
            return;
        }
    };

    println!("Test: {}", luhns(long_no));
}

fn luhns(mut number: u64) -> u64 {
    let mut sum: u64 = 0;
    let mut double = false;

    while number > 0 {
        let mut digit = number % 10;

        if double {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        sum += digit;
        number /= 10;
        double = !double;
    }
    sum
}
