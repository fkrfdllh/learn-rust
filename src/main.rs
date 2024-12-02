#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

/**
 * Installing external crates
 * Crates === Packages
 * 
 * cargo add <package name>
 */

impl Deck {
    fn new() -> Self {
        let suits  = vec!["Hearts", "Spades", "Diamonds"];
        let values = vec!["Ace", "Two", "Three"];
       
        let mut cards = vec![];
    
        for suit in suits {
            for value in &values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        
        Deck { cards }
    }
}

fn main() {
    let deck = Deck::new();
    
    println!("Here's your deck: {:#?}", deck);
}
