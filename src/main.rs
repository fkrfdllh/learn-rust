fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
        String::from("yellow"),
        String::from("white"),
        String::from("black"),
    ];

    print_elements(&colors[1..4]);
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
