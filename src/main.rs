#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, author: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, author } = self {
        //     format!("Movie: {} {}", title, author)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media description")
        // }

        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, author } => format!("Movie: {} {}", title, author),
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
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

    println!("{}", movie.description());
    println!("{}", book.description());
    println!("{}", audiobook.description());

    print_media(movie);
    print_media(book);
    print_media(audiobook);
}
