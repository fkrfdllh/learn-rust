fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
        String::from("yellow"),
        String::from("white"),
        String::from("black"),
    ];

    print_elements(&colors);
}

fn print_elements(elements: &Vec<String>) {
    // for element in elements {
    //     println!("{}", element)
    // }

    // pipes (|param|) inside of bracket is anonymous function
    elements.iter().for_each(|element| println!("{}", element));
}
