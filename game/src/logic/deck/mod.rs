use encryption::{EncryptedCard, EncryptedCards};

pub mod encryption;
pub mod draw;

// Use to received partial encrypted deck from multiple player
// when are parts are here we can do the final shuffle
// and produce the final deck
pub struct DeckBuilder {

}

#[derive(Default)]
pub struct Deck {
    pub cards: EncryptedCards,
}

impl Deck {

    pub fn create_default_deck() -> Self {
        Self { cards: (1..=10).map(|i| (i as u32).to_le_bytes().to_vec()).collect() }
    }

    pub fn create(cards: EncryptedCards) -> Self {
        Self { cards: cards }
    }

    pub fn add_encrypted_card_from_player(&mut self, cards: &mut EncryptedCards) {
        self.cards.append(cards);
    }

    pub fn draw_cards(&mut self, quantity: u32) -> Result<Vec<EncryptedCard>, ()> {
        if quantity > (self.cards.len() as u32) {
            eprintln!("deck would run out error");
            return Err(());
        }

        return Ok(self.cards.split_off( self.cards.len() - (quantity as usize)));
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_card() {
        let mut deck = Deck::create_default_deck();


        let initial_length = deck.cards.len();

        let cards = deck.draw_cards(2).unwrap();


        assert_eq!(initial_length - 2, deck.cards.len());
        assert_eq!(2, cards.len());

        assert_eq!((9 as u32).to_le_bytes().to_vec(), cards.get(0).unwrap().clone());
        assert_eq!((10 as u32).to_le_bytes().to_vec(), cards.get(1).unwrap().clone());
    }
}

