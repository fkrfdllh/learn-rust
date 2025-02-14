/* Instead of doing this:
struct Book {
    title: String,
    author: String,
}

struct Movie {
    title: String,
    director: String,
}

struct Audiobook {
    title: String,
}
*/

// Do this:
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, author: String },
    Audiobook { title: String },
}

fn main() {
    println!("Hello, world!");
}
