use std::io;

fn main() {
    println!("How tall is your tower?");

    let height: usize = loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        match input.trim().parse() {
            Ok(num) => {
                if num > 0 {
                    break num;
                } else {
                    println!("Please enter a positive integer");
                }
            }

            Err(_) => {
                println!("Please enter valid type: Unsigned Integer")
            }
        }
    };

    for i in 1..=height {
        println!(
            "{}{}{}{}",
            " ".repeat(height - i),
            "#".repeat(i),
            "  ",
            "#".repeat(i)
        );
    }
}
