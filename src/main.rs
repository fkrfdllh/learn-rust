#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, author: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: {} {}", title, author),
            Media::Movie { title, author } => format!("Movie: {} {}", title, author),
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Podcast(id) => format!("Podcast: {}", id),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
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
    let podcast = Media::Podcast(1);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);
}
