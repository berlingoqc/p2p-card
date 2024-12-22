use crate::utils::splice;

use super::{encryption::EncryptedCard, Deck};



pub struct DrawDecryptionRequest {
    cards: Vec<EncryptedCard>,
    config: DrawConfig,
}


pub struct DrawProcess {
    cards: Vec<EncryptedCard>,
    config: DrawConfig,
}


pub struct DrawConfig {
    pub quantity: usize,
    pub drawer_id: String,
    pub layer_ids: Vec<String>,
}


impl DrawProcess {
    pub fn start(deck: &mut Deck, config: DrawConfig) -> Self {
        let cards = if (config.quantity > deck.cards.len()) {
            deck.cards.clone()
        } else {
            splice(&mut deck.cards, 0, config.quantity)
        };

        Self { cards: cards, config }
    }

    // Ask the next player to remove it's encrypted layer
    pub fn ask_for_layer_removal(&mut self) -> Result<Vec<EncryptedCard>, ()>  {

        return Err(());
    }

    pub fn get_private_card(&self) -> Result<Vec<EncryptedCard>, ()> {

        return Err(());
    }
}



impl DrawDecryptionRequest {

    pub fn start(deck: &mut Deck, config: DrawConfig) -> Self {
        let cards = if (config.quantity > deck.cards.len()) {
            deck.cards.clone()
        } else {
            splice(&mut deck.cards, 0, config.quantity)
        };

        Self { cards: cards, config }
    }
}