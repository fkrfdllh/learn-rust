fn main() {
    let languages = vec![
        String::from("PHP"),
        String::from("JavaScript"),
        String::from("Java"),
        String::from("Golang"),
        String::from("Rust"),
        String::from("Python"),
    ];

    let result = next_language(&languages, "Python");

    println!("{}", result);
}

// lifetime annotation used to mark
// the output of the scope gonna use which reference
// it will use
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
