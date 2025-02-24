use std::fs;

fn main() {
    match fs::read_to_string("./logs.txt") {
        Ok(value) => {
            let errors = extract_errors(value.as_str());

            println!("{:#?}", errors);
        }
        Err(message) => println!("failed to read file: {}", message),
    }
}

fn extract_errors(text: &str) -> Vec<&str> {
    let splitted_text = text.split("\n");

    let mut results = vec![];

    for line in splitted_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    return results;
}
