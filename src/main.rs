fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
        String::from("yellow"),
        String::from("white"),
        String::from("black"),
    ];

    // print_elements(&colors[1..4]);
    // shorten_strings(&mut colors);
    // let uppercased = to_uppercase(&colors);

    // let mut dest = vec![];

    // move_elements(colors, &mut dest);

    // let exploded = explode(&colors);

    let find_one = find_color_or(&colors, "fi", "no color");

    println!("{:#?}", find_one);
}

fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element)
    // }

    elements
        .iter()
        .map(|element| format!("{} {}", element, element))
        .for_each(|element| println!("{}", element));
}

fn shorten_strings(elements: &mut Vec<String>) {
    elements
        .iter_mut()
        .for_each(|elements| elements.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    return elements
        .iter()
        .map(|element| element.to_uppercase())
        .collect();
}

fn move_elements(source_vec: Vec<String>, dest_vec: &mut Vec<String>) {
    source_vec
        .into_iter()
        .for_each(|element| dest_vec.push(element));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    return elements
        .iter()
        .map(|element| element.chars().map(|char| char.to_string()).collect())
        .collect();
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    // let result = elements.iter().find(|element| element.contains(search));

    // return match result {
    //     Some(value) => value.to_string(),
    //     None => fallback.to_string(),
    // };

    return elements
        .iter()
        .find(|element| element.contains(search))
        .map_or(String::from(fallback), |element| element.to_string());
}
