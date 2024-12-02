#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

/*
 * array vs Vector
 * array just using `[]`
 * vector using `vec![]`
 * 
 * array is in fixed size
 * vector's size can be dynamic
 */

fn main() {
    let suits  = vec!["Hearts", "Spades", "Diamonds"];
    let values = vec!["Ace", "Two", "Three"];
    
    /*
     * let variable = value <-- this is immutable, means that variable can't be reassigned (just like constant variable)
     * let mut variable = value <-- this variable is mutable, the value can be changed
     */
    
    let mut cards = vec![];
    
    for suit in suits {
        for value in &values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }
    
    // let deck = Deck { cards: cards };
    let deck = Deck { cards };
    
    println!("Here's your deck: {:#?}", deck);
}
