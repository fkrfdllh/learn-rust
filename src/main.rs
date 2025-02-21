use std::fs;

fn main() {
    match fs::read_to_string("./logs.txt") {
        Ok(value) => println!("{}", value.len()),
        Err(message) => println!("failed to read file: {}", message),
    }
}
