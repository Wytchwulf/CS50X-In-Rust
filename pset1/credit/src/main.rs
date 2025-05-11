use std::io;

fn main() {
    // Take input from user
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    // Sort Card Type
    let input = input.trim();

    let card_type: &str = match input.len() {
        13 => {
            if &input[0..1] == "4" {
                "VISA"
            } else {
                "INVALID"
            }
        }
        15 => match &input[0..2] {
            "34" => "AMEX",
            "37" => "AMEX",
            _ => "INVALID",
        },
        16 => match &input[0..1] {
            "4" => "VISA",
            _ => match &input[0..2] {
                "51" => "MASTERCARD",
                "52" => "MASTERCARD",
                "53" => "MASTERCARD",
                "54" => "MASTERCARD",
                "55" => "MASTERCARD",
                _ => "INVALID",
            },
        },
        _ => "INVALID",
    };

    // Convert to int
    let long_no: u64 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Could not convert to integer");
            return;
        }
    };

    // TEST
    println!("Test: {} \n{}", luhns(long_no), card_type)
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
