use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    // let text = fs::read_to_string("./logs.txt").expect("failed to read logs.txt");
    // let errors = extract_errors(text.as_str());
    // fs::write("./logs/error.log", errors.join("\n")).expect("failed to write error.log");

    let text = fs::read_to_string("./logs.txt")?;
    let errors = extract_errors(text.as_str());
    fs::write("./error.log", errors.join("\n"))?;

    Ok(())
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
