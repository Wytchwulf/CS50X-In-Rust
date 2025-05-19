use std::io;

fn main() {
    // Take input from user
    println!("Please enter Card no.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    // Sort Card Type
    let input = input.trim();

    println!("Card No. : {}", &input);

    let card_type: &str = match input.len() {
        13 if input.starts_with("4") => "VISA",
        15 if &input[0..2] == "34" || &input[0..2] == "37" => "AMEX",
        16 if input.starts_with("4") => "VISA",
        16 if matches!(&input[0..2], "51" | "52" | "53" | "54" | "55") => "MASTERCARD",
        _ => "INVALID",
    };

    // Convert to int
    let long_no: u64 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("INVALID");
            return;
        }
    };

    // Validation Check
    let valid = luhns(long_no);
    if card_type == "INVALID" || valid % 10 != 0 {
        println!("Invalid Card Type");
        return;
    } else {
        println!("{}", card_type);
    };
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
