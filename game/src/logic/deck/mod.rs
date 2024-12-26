pub mod encryption;
pub mod draw;


#[derive(Clone, Copy)]
pub struct DeckCard {
    number: u16,
}

#[derive(Clone)]
pub struct StartingDeck {
    cards: Vec<DeckCard>
}

pub struct Deck {
    cards: Vec<Vec<u8>>,


    encryption_order: Vec<String>
}



impl DeckCard {

    fn from_u32(value: u32) -> Self {
        DeckCard { number: (value >> 16) as u16}
    }

    fn to_u32(&self) -> u32 {
        ((self.number as u32) << 16)
    }

}

impl StartingDeck {

    pub fn create_default_deck() -> Self {
        StartingDeck { cards: vec![
            DeckCard { number: 1},
            DeckCard { number: 2},
            DeckCard { number: 3},
            DeckCard { number: 4},
            DeckCard { number: 5},
            DeckCard { number: 6},
            DeckCard { number: 7},
            DeckCard { number: 8},
            DeckCard { number: 9},
        ] }
    }
}




impl Deck {
    fn builder(startin_deck: StartingDeck) -> Self {
        Deck { cards: startin_deck.cards.iter().map(|c| c.to_u32().to_le_bytes().to_vec()).collect(), encryption_order: vec![] }
    }
}

