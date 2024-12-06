
pub struct DeckCardConfiguration {

}

pub struct DeckConfiguration {
    cards: Vec<DeckCardConfiguration>
}

pub struct Deck {
    cards: Vec<u32>

}


impl Deck {
    fn builder(config: &DeckConfiguration) -> Self {

        return Deck { cards:vec![] }

    }
}