fn main() {
    println!("Hello, What is your name?");

   let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Could not read line.");

    println!("Hello, {}", name);
}
