#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, author: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: {} {}", title, author),
            Media::Movie { title, author } => format!("Movie: {} {}", title, author),
            Media::Audiobook { title } => format!("Audiobook: {}", title),
            Media::Podcast(id) => format!("Podcast: {}", id),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}
