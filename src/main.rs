fn main() {
    let languages = vec![
        String::from("PHP"),
        String::from("JavaScript"),
        String::from("Java"),
        String::from("Golang"),
        String::from("Rust"),
        String::from("Python"),
    ];

    let next_language = next_language(&languages, "PHP");
    let last_language = last_language(&languages);

    println!("Next Language: {}", next_language);
    println!("Last Language: {}", last_language);
}

fn next_language<'a, 'b>(languages: &'a [String], current: &'b str) -> &'a str {
    let mut found = false;

    for language in languages {
        if found {
            return language;
        }

        if language == current {
            found = true;
        }
    }

    return languages.last().unwrap();
}

fn last_language(languages: &[String]) -> &str {
    return languages.last().unwrap();
}
