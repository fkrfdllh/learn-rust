#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, author: String },
    Audiobook { title: String },
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

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

    print_media(movie);
    print_media(book);
    print_media(audiobook);
}
