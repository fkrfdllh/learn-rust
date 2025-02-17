mod content;

use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An audiobook"),
    };
    let movie = Media::Movie {
        title: String::from("Good movie"),
        author: String::from("Good director"),
    };
    let book = Media::Book {
        title: String::from("Bad book"),
        author: String::from("Bad book"),
    };
    let podcast = Media::Podcast(1);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(100);
    let placeholder = Media::Placeholder;

    // Using .unwrap() to debugging
    // because it only shows Some option
    // if the value is None then it will panic
    // println!("{:#?}", item.unwrap());

    // Using .expect will panic too
    // but this good for testing
    // because it is returns message inside expect parameter
    // println!("{:#?}", item.expect("expected there to be an item here"));

    // Using .unwrap_or gonna return Some
    // if None so it will return what is inside in parameter (fallback value)
    println!("{:#?}", item.unwrap_or(&placeholder));
}
