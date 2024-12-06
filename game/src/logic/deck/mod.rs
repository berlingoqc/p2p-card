use super::cards::card::Card;


pub struct DeckConfiguration {

}

pub struct Deck {
    cards: Vec<Box<dyn Card>>
}



impl Deck {
    fn builder(config: &DeckConfiguration) -> Self {


        return Deck { cards:vec![] }

    }
}