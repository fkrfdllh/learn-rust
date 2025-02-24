use std::fs;

fn main() {
    let mut errors = vec![];

    match fs::read_to_string("./logs.txt") {
        Ok(value) => {
            errors = extract_errors(value.as_str());
        }
        Err(message) => println!("failed to read file: {}", message),
    }

    println!("{:#?}", errors);
}

fn extract_errors(text: &str) -> Vec<String> {
    let splitted_text = text.split("\n");

    let mut results = vec![];

    for line in splitted_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    return results;
}
