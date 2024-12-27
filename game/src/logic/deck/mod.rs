use encryption::{EncryptedCard, EncryptedCards};

pub mod encryption;
pub mod draw;


#[derive(Clone, Copy)]
pub struct DeckCard {
    number: u16,
}

#[derive(Clone)]
pub struct StartingDeck {
    pub cards: Vec<DeckCard>
}

pub struct PartialDeck {
    pub cards: Vec<Vec<u8>>
}

#[derive(Default)]
pub struct Deck {
    pub cards: Vec<Vec<u8>>,
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

    pub fn to_vec(&self) -> Vec<u32> {
        self.cards.iter().map(|v| v.to_u32()).collect()
    }
}




impl Deck {

    pub fn add_encrypted_card_from_player(&mut self, cards: &mut EncryptedCards) {
        self.cards.append(cards);
    }

    pub fn draw_cards(&mut self, quantity: u32) -> Result<Vec<EncryptedCard>, ()> {
        if quantity > (self.cards.len() as u32) {
            eprintln!("deck would run out error");
            return Err(());
        }

        return Ok(vec![]);
    }
}

