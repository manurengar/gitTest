use std::io::{self, Write};

fn main() {
    print!("Enter your age: ");
    io::stdout().flush().expect("Error when flushing");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read from terminal");

    let age: u32 = user_input.trim().parse().expect("Type a valid number!");
    println!("\nYour age is {age}");
}
