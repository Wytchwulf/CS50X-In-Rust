use std::io; // Import for stdin

fn main() {
    println!("How tall is your tower?");

    let height: usize = loop {
        // Let height = the number that breaks the loop
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");
        // Create a mutable empty instance of a string and read user input into it.

        match input.trim().parse() {
            // Remove whitespace and convert input string to u32
            Ok(num) => {
                if num > 0 {
                    break num;
                    // If convertable to u32 and more than 0 then break loop and return num
                } else {
                    println!("Please enter a positive number");
                }
            }
            Err(_) => {
                println!("Please enter a positive number");
                // If unconvertable to u32 print message and continue loop
            }
        }
    };

    for block in (0..height) {
        println!("{}", "#".repeat(height));
    }
}
