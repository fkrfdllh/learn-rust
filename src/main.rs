use std::fs;

fn main() {
    // match fs::read_to_string("./logs.txt") {
    //     Ok(value) => {
    //         let errors = extract_errors(value.as_str());

    //         match fs::write("./error.log", errors.join("\n")) {
    //             Ok(..) => println!("error log stored"),
    //             Err(error) => println!("failed to write error log caused by: {}", error),
    //         }
    //     }
    //     Err(err) => println!("failed to read file: {}", err),
    // }

    // these one is good to go too
    // but instead of handling error, it will throw panic
    let text = fs::read_to_string("./logs.txt").expect("failed to read logs.txt");
    let errors = extract_errors(text.as_str());
    fs::write("./logs/error.log", errors.join("\n")).expect("failed to write error.log");
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
