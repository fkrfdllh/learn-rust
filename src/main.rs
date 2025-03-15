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

    /*
     * !! TLDR !!
     * .map() is adaptor of iteration
     * .for_each() is consumer of iteration
     * iteration can't ended with adaptor, so it must be ended with consumer
     * so, if we wanna use adaptor then chain them with consumer
     */
    elements
        .iter()
        .map(|element| format!("{} {}", element, element))
        .for_each(|element| println!("{}", element));
}
