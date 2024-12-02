#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

/**
 * impl means implementation
 * implementation means to add function into struct
 * or what we called class function
 * 
 * fn functionName() -> Self
 * `->` means return type of the method/function
 * `-> Self` means return of the functionName() is Self
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
    
        // return Deck { cards };
        Deck { cards }
    }
}

fn main() {
    let deck = Deck::new();
    
    println!("Here's your deck: {:#?}", deck);
}
